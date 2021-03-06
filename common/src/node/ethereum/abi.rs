#![cfg(feature = "abi-contract")]

use crate::EthError;
use ethers::prelude::abi::{Error, ParamType, Token};
use ethers::prelude::{Address, H160, U256};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

type EthAbiFieldName = String;
type EthAbiStructName = String;

/// Ethereum ABI parameter type
#[derive(Debug, Eq, PartialEq)]
pub enum EthAbiParamType {
    Address,
    Bytes,
    Int(usize),
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
impl From<&str> for EthAbiParamType {
    fn from(iden: &str) -> Self {
        if let Some(param_type) = parse_param_type_array(iden) {
            return param_type;
        }
        if let Some(param_type) = parse_param_type_fixed_array(iden) {
            return param_type;
        }

        match iden.trim() {
            "address" => EthAbiParamType::Address,
            "bool" => EthAbiParamType::Bool,
            "bytes" => EthAbiParamType::Bytes,
            "h160" => EthAbiParamType::FixedBytes(20),
            "h256" | "secret" | "hash" => EthAbiParamType::FixedBytes(32),
            "h512" | "public" => EthAbiParamType::FixedBytes(64),
            "int256" | "int" | "uint" | "uint256" => EthAbiParamType::Uint(256),
            "string" => EthAbiParamType::String,
            iden => parse_param_type_fixed_bytes(iden)
                .or_else(|| parse_param_type_int(iden))
                .or_else(|| parse_param_type_uint(iden))
                .or_else(|| parse_param_type_from_abbreviated_integer(iden))
                .unwrap_or_else(|| EthAbiParamType::Struct(iden.to_owned())),
        }
    }
}

impl TryFrom<&EthAbiParamType> for ParamType {
    type Error = EthError;

    fn try_from(param_type: &EthAbiParamType) -> Result<Self, Self::Error> {
        Ok(match param_type {
            EthAbiParamType::Address => ParamType::Address,
            EthAbiParamType::Bytes => ParamType::Bytes,
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
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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
            EthAbiToken::Array(values) => Token::FixedArray(
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

/// Parse a string to parameter type Array, return None otherwise.
fn parse_param_type_array(iden: &str) -> Option<EthAbiParamType> {
    lazy_static! {
        // e.g. uint256[] or Person[]
        static ref ARRAY_REGEX: Regex = Regex::new(r"\A(.+)\[\]\z").unwrap();
    }

    let captures = ARRAY_REGEX.captures(iden.trim())?;
    let array_type = captures.get(1)?.as_str().into();

    Some(EthAbiParamType::Array(Box::new(array_type)))
}

/// Parse a string to parameter type FixedArray, return None otherwise.
fn parse_param_type_fixed_array(iden: &str) -> Option<EthAbiParamType> {
    lazy_static! {
        // e.g. uint256[100] or Person[100]
        static ref FIXED_ARRAY_REGEX: Regex = Regex::new(r"\A(.+)\[(\d+)\]\z").unwrap();
    }

    let captures = FIXED_ARRAY_REGEX.captures(iden.trim())?;
    let array_type = captures.get(1)?.as_str().into();
    let array_size = captures.get(2)?.as_str().parse::<usize>().ok()?;

    Some(EthAbiParamType::FixedArray(
        Box::new(array_type),
        array_size,
    ))
}

/// Parse a string to parameter type FixedBytes with specified size, return None otherwise.
fn parse_param_type_fixed_bytes(iden: &str) -> Option<EthAbiParamType> {
    let prefix = "bytes";
    if !iden.starts_with(prefix) {
        return None;
    }
    let size = iden
        .chars()
        .skip(prefix.len())
        .collect::<String>()
        .parse::<usize>()
        .ok()?;
    Some(EthAbiParamType::FixedBytes(size))
}

/// Parse a string to parameter type Int or Uint from abbreviated integer and specified size, return
/// None otherwise.
fn parse_param_type_from_abbreviated_integer(iden: &str) -> Option<EthAbiParamType> {
    let size = iden
        .chars()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .ok()?;
    if iden.starts_with('i') {
        Some(EthAbiParamType::Int(size))
    } else if iden.starts_with('u') {
        Some(EthAbiParamType::Uint(size))
    } else {
        None
    }
}

/// Parse a string to parameter type Int with specified size, return None otherwise.
fn parse_param_type_int(iden: &str) -> Option<EthAbiParamType> {
    let prefix = "int";
    if !iden.starts_with(prefix) {
        return None;
    }
    let size = iden
        .chars()
        .skip(prefix.len())
        .collect::<String>()
        .parse::<usize>()
        .ok()?;
    Some(EthAbiParamType::Int(size))
}

/// Parse a string to parameter type Uint with specified size, return None otherwise.
fn parse_param_type_uint(iden: &str) -> Option<EthAbiParamType> {
    let prefix = "uint";
    if !iden.starts_with(prefix) {
        return None;
    }
    let size = iden
        .chars()
        .skip(prefix.len())
        .collect::<String>()
        .parse::<usize>()
        .ok()?;
    Some(EthAbiParamType::Uint(size))
}
