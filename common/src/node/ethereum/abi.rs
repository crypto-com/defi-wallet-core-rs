#![cfg(feature = "abi-contract")]

use crate::EthError;
use ethers::prelude::abi::{Error, ParamType, Token};
use ethers::prelude::{Address, H160, U256};
use pest::Parser;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, mem, num::NonZeroUsize, str::FromStr};
type EthAbiFieldName = String;
type EthAbiStructName = String;

/// Ethereum ABI parameter type
/// based on https://docs.soliditylang.org/en/develop/abi-spec.html#types
/// without fixed-point decimal numbers which aren't supported by the latest
/// Solidity compilers.
/// Plus `Struct` for EIP-712
#[derive(Debug, Eq, PartialEq)]
pub enum EthAbiParamType {
    Address,
    Bytes,
    Function,
    IntAlias,
    Int(usize),
    UintAlias,
    Uint(usize),
    Bool,
    String,
    Array(Box<EthAbiParamType>),
    FixedBytes(usize),
    FixedArray(Box<EthAbiParamType>, usize),
    Tuple(Vec<EthAbiParamType>),
    /// Above are standard Solidity types. This struct type is used to extend for recursively
    /// nested structs.
    Struct(EthAbiStructName),
}

mod parser {
    use crate::EthError;
    use ethers::prelude::abi;
    use pest::iterators::Pairs;
    use pest_derive::Parser;

    use super::EthAbiParamType;

    #[derive(Parser)]
    #[grammar_inline = r#"
    // m should be a multiple of 8 and <= 256
    m_param = { ASCII_NONZERO_DIGIT ~ ASCII_DIGIT{0, 2} }
    m_short_param = { "3" ~ '0'..'2' | ("1" | "2") ~ ASCII_DIGIT | ASCII_NONZERO_DIGIT }
    int_param = { ("u")? ~ "int" ~ m_param? }
    alias_param = { "address" | "bool" | "function" | "string" | (ASCII_ALPHA_UPPER ~ ASCII_ALPHANUMERIC*) }
    bytes_param = { "bytes" ~ m_short_param? }
    array_size_no = { (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)? }
    array_size_param = _{ "[" ~ array_size_no? ~ "]" }
    non_recur_param = _{ (int_param | bytes_param | alias_param ) }
    base_param = { (non_recur_param | tuple_param) ~ array_size_param* }
    tuple_param = { "()" | "(" ~ (abi_param ~ ("," ~ abi_param)*)? ~ ")" }
    abi_param = _{ (base_param | tuple_param ) }
    "#]
    pub struct AbiParser;

    fn maybe_array(inner_iter: &mut Pairs<Rule>) -> Result<EthAbiParamType, EthError> {
        let mut result = from_parse_tree(inner_iter)?;

        for arrays in inner_iter.by_ref() {
            let msize = arrays.as_str().parse::<usize>();
            if let Ok(size) = msize {
                result = EthAbiParamType::FixedArray(Box::new(result), size);
            } else {
                result = EthAbiParamType::Array(Box::new(result));
            }
        }
        Ok(result)
    }

    pub(crate) fn from_parse_tree(
        parse_tree: &mut Pairs<Rule>,
    ) -> Result<EthAbiParamType, EthError> {
        let first = parse_tree.next().unwrap();

        match first.as_rule() {
            // abi_param, non_recur_param, and array_size_param are silent (never appear in the parse tree)
            // m_param, m_short_param, array_size_no are always extracted with their corresponding non-terminal
            // (int_param, bytes, and base_param / tuple_param)
            Rule::abi_param
            | Rule::non_recur_param
            | Rule::array_size_param
            | Rule::m_param
            | Rule::m_short_param
            | Rule::array_size_no => Err(EthError::AbiError(abi::Error::Other(
                "Unexpected parse tree item".into(),
            ))),

            Rule::base_param => {
                let mut inner_iter = first.into_inner();
                maybe_array(&mut inner_iter)
            }
            Rule::int_param => {
                let uint = first.as_str().starts_with('u');
                let inner = first.into_inner();
                let m_param_raw = inner.as_str();
                let m_param = m_param_raw.parse::<usize>();
                match m_param {
                    Ok(m) if m % 8 == 0 && m <= 256 && m > 0 => {
                        if uint {
                            Ok(EthAbiParamType::Uint(m))
                        } else {
                            Ok(EthAbiParamType::Int(m))
                        }
                    }
                    Err(_) if m_param_raw.is_empty() => {
                        if uint {
                            Ok(EthAbiParamType::UintAlias)
                        } else {
                            Ok(EthAbiParamType::IntAlias)
                        }
                    }
                    _ => Err(EthError::AbiError(abi::Error::InvalidData)),
                }
            }
            Rule::alias_param => {
                let alias = first.as_str();
                Ok(match alias {
                    "address" => EthAbiParamType::Address,
                    "bool" => EthAbiParamType::Bool,
                    "string" => EthAbiParamType::String,
                    "function" => EthAbiParamType::Function,
                    _ => EthAbiParamType::Struct(alias.to_string()),
                })
            }
            Rule::bytes_param => {
                let inner = first.into_inner();
                let m_param = inner.as_str();
                Ok(if let Ok(size) = m_param.parse::<usize>() {
                    EthAbiParamType::FixedBytes(size)
                } else {
                    EthAbiParamType::Bytes
                })
            }

            Rule::tuple_param => {
                let internals = first
                    .into_inner()
                    .flat_map(|x| maybe_array(&mut x.into_inner()))
                    .collect();
                Ok(EthAbiParamType::Tuple(internals))
            }
        }
    }
}

impl EthAbiParamType {
    pub fn iter(&self) -> EthAbiParamTypeIter<'_> {
        EthAbiParamTypeIter {
            children: std::slice::from_ref(self),
            parent: None,
        }
    }
}

/// Iterator for EthAbiParamType
#[derive(Default)]
pub struct EthAbiParamTypeIter<'a> {
    children: &'a [EthAbiParamType],
    parent: Option<Box<EthAbiParamTypeIter<'a>>>,
}

impl<'a> Iterator for EthAbiParamTypeIter<'a> {
    type Item = &'a EthAbiParamType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(EthAbiParamType::Tuple(children)) => {
                self.children = &self.children[1..];

                // start iterating the child trees
                *self = EthAbiParamTypeIter {
                    children: children.as_slice(),
                    parent: Some(Box::new(mem::take(self))),
                };
                self.next()
            }
            Some(EthAbiParamType::Array(child)) | Some(EthAbiParamType::FixedArray(child, _)) => {
                self.children = &self.children[1..];
                *self = EthAbiParamTypeIter {
                    children: std::slice::from_ref(child),
                    parent: Some(Box::new(mem::take(self))),
                };
                self.next()
            }
            Some(l) => {
                self.children = &self.children[1..];
                Some(l)
            }
        }
    }
}

/// Implementing Display trait is used to encode the parameter type. It is implemented for `Array`,
/// `FixedArray`, `Struct` and `Tuple`, since Struct is a custom type, and other parameter types
/// could has item type of a Struct.
impl fmt::Display for EthAbiParamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(item_param_type) => write!(f, "{item_param_type}[]"),
            Self::FixedArray(item_param_type, array_size) => {
                write!(f, "{item_param_type}[{array_size}]")
            }
            Self::Struct(struct_name) => write!(f, "{struct_name}"),
            Self::Tuple(item_param_types) => {
                let formatted_types = item_param_types
                    .iter()
                    .map(|t| format!("{t}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "({formatted_types})")
            }
            Self::Function => write!(f, "function"),
            Self::IntAlias => write!(f, "int"),
            Self::UintAlias => write!(f, "uint"),
            _ => {
                let param_type = ParamType::try_from(self).map_err(|_| fmt::Error)?;
                write!(f, "{param_type}")
            }
        }
    }
}
/// Parse a string to Ethereum ABI parameter type.
/// This function references code of
/// [find_parameter_type](https://docs.rs/ethers/latest/ethers/?search=find_parameter_type)
/// in ethers-rs.
impl FromStr for EthAbiParamType {
    type Err = EthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pest::set_call_limit(Some(NonZeroUsize::new(1000).unwrap()));
        let mut parse_tree = parser::AbiParser::parse(parser::Rule::abi_param, s)
            .map_err(|_e| EthError::AbiError(Error::InvalidData))?;
        let iden = parser::from_parse_tree(&mut parse_tree)?;
        Ok(iden)
    }
}

impl TryFrom<&EthAbiParamType> for ParamType {
    type Error = EthError;

    fn try_from(param_type: &EthAbiParamType) -> Result<Self, Self::Error> {
        Ok(match param_type {
            EthAbiParamType::Address => ParamType::Address,
            EthAbiParamType::Bytes => ParamType::Bytes,
            EthAbiParamType::UintAlias => ParamType::Uint(256),
            EthAbiParamType::IntAlias => ParamType::Int(256),
            EthAbiParamType::Int(size) => ParamType::Int(*size),
            EthAbiParamType::Uint(size) => ParamType::Uint(*size),
            EthAbiParamType::Bool => ParamType::Bool,
            EthAbiParamType::String => ParamType::String,
            EthAbiParamType::Array(boxed_param_type) => {
                ParamType::Array(Box::new(ParamType::try_from(boxed_param_type.as_ref())?))
            }
            EthAbiParamType::FixedBytes(size) => ParamType::FixedBytes(*size),
            EthAbiParamType::FixedArray(boxed_param_type, size) => ParamType::FixedArray(
                Box::new(ParamType::try_from(boxed_param_type.as_ref())?),
                *size,
            ),
            EthAbiParamType::Tuple(params) => ParamType::Tuple(
                params
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            EthAbiParamType::Function => ParamType::FixedBytes(24),
            EthAbiParamType::Struct(struct_name) => {
                return Err(Error::Other(
                    format!("Unsupported nested struct conversion: {struct_name}").into(),
                )
                .into());
            }
        })
    }
}

/// Ethereum ABI token
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone)]
pub enum EthAbiToken {
    Address(H160),
    FixedBytes(Vec<u8>),
    Bytes(Vec<u8>),
    Int(U256),
    Uint(U256),
    Bool(bool),
    String(String),
    FixedArray(Vec<EthAbiToken>),
    Array(Vec<EthAbiToken>),
    Tuple(Vec<EthAbiToken>),
    /// Above are standard Solidity values. This struct value is used to extend for recursively
    /// nested structs.
    Struct(EthAbiStructName, HashMap<EthAbiFieldName, EthAbiToken>),
}

impl EthAbiToken {
    /// Create from a string of address.
    pub fn from_address_str(address_str: &str) -> Result<Self, EthError> {
        Ok(Self::Address(
            Address::from_str(address_str).map_err(|_| EthError::HexConversion)?,
        ))
    }

    /// Create from a string of signed integer.
    pub fn from_int_str(int_str: &str) -> Result<Self, EthError> {
        Ok(Self::Int(
            U256::from_dec_str(int_str).map_err(|_| EthError::HexConversion)?,
        ))
    }

    /// Create from a string of unsigned integer.
    pub fn from_uint_str(uint_str: &str) -> Result<Self, EthError> {
        Ok(Self::Uint(
            U256::from_dec_str(uint_str).map_err(|_| EthError::HexConversion)?,
        ))
    }
}

impl TryFrom<&EthAbiToken> for Token {
    type Error = EthError;

    fn try_from(token: &EthAbiToken) -> Result<Self, Self::Error> {
        Ok(match token {
            EthAbiToken::Address(value) => Token::Address(*value),
            EthAbiToken::FixedBytes(value) => Token::FixedBytes(value.clone()),
            EthAbiToken::Bytes(value) => Token::Bytes(value.clone()),
            EthAbiToken::Int(value) => Token::Int(*value),
            EthAbiToken::Uint(value) => Token::Uint(*value),
            EthAbiToken::Bool(value) => Token::Bool(*value),
            EthAbiToken::String(value) => Token::String(value.clone()),
            EthAbiToken::FixedArray(values) => Token::FixedArray(
                values
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            EthAbiToken::Array(values) => Token::Array(
                values
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            EthAbiToken::Tuple(values) => Token::Tuple(
                values
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            ),
            EthAbiToken::Struct(struct_name, _struct_fields) => {
                return Err(Error::Other(
                    format!("Unsupported nested struct conversion: {struct_name}").into(),
                )
                .into());
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::abi::EthAbiParamType;

    #[test]
    pub fn test_parser() {
        let t: EthAbiParamType = "uint256".parse().unwrap();
        assert_eq!(t, EthAbiParamType::Uint(256));
        let t: EthAbiParamType = "uint256[]".parse().unwrap();
        assert_eq!(
            t,
            EthAbiParamType::Array(Box::new(EthAbiParamType::Uint(256)))
        );
        let t: EthAbiParamType = "uint256[100]".parse().unwrap();
        assert_eq!(
            t,
            EthAbiParamType::FixedArray(Box::new(EthAbiParamType::Uint(256)), 100)
        );
        let t: EthAbiParamType = "uint256[100][]".parse().unwrap();
        assert_eq!(
            t,
            EthAbiParamType::Array(Box::new(EthAbiParamType::FixedArray(
                Box::new(EthAbiParamType::Uint(256)),
                100
            )))
        );
        let t: EthAbiParamType = "(uint256[100][100],address)".parse().unwrap();
        assert_eq!(
            t,
            EthAbiParamType::Tuple(vec![
                EthAbiParamType::FixedArray(
                    Box::new(EthAbiParamType::FixedArray(
                        Box::new(EthAbiParamType::Uint(256)),
                        100
                    )),
                    100
                ),
                EthAbiParamType::Address
            ])
        );
        let t: EthAbiParamType = "(uint256[100][100],address)[]".parse().unwrap();
        assert_eq!(
            t,
            EthAbiParamType::Array(Box::new(EthAbiParamType::Tuple(vec![
                EthAbiParamType::FixedArray(
                    Box::new(EthAbiParamType::FixedArray(
                        Box::new(EthAbiParamType::Uint(256)),
                        100
                    )),
                    100
                ),
                EthAbiParamType::Address
            ])))
        );
        let t: EthAbiParamType = "(uint[100][100],((),address))[]".parse().unwrap();
        assert_eq!(
            t,
            EthAbiParamType::Array(Box::new(EthAbiParamType::Tuple(vec![
                EthAbiParamType::FixedArray(
                    Box::new(EthAbiParamType::FixedArray(
                        Box::new(EthAbiParamType::UintAlias),
                        100
                    )),
                    100
                ),
                EthAbiParamType::Tuple(vec![
                    EthAbiParamType::Tuple(vec![]),
                    EthAbiParamType::Address
                ])
            ])))
        );
    }
}
