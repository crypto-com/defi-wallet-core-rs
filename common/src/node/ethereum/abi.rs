#![cfg(feature = "abi-contract")]

use crate::EthError;
use ethers::prelude::abi::{Error, ParamType, Token};
use ethers::prelude::{Address, H160, U256};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
}

impl From<&EthAbiParamType> for ParamType {
    fn from(param_type: &EthAbiParamType) -> Self {
        match param_type {
            EthAbiParamType::Address => ParamType::Address,
            EthAbiParamType::Bytes => ParamType::Bytes,
            EthAbiParamType::Int(size) => ParamType::Int(*size),
            EthAbiParamType::Uint(size) => ParamType::Uint(*size),
            EthAbiParamType::Bool => ParamType::Bool,
            EthAbiParamType::String => ParamType::String,
            EthAbiParamType::Array(boxed_param_type) => {
                ParamType::Array(Box::new(ParamType::from(boxed_param_type.as_ref())))
            }
            EthAbiParamType::FixedBytes(size) => ParamType::FixedBytes(*size),
            EthAbiParamType::FixedArray(boxed_param_type, size) => {
                ParamType::FixedArray(Box::new(ParamType::from(boxed_param_type.as_ref())), *size)
            }
            EthAbiParamType::Tuple(params) => {
                ParamType::Tuple(params.iter().map(Into::into).collect())
            }
        }
    }
}

/// Parse a string to Ethereum ABI parameter type.
/// This function references code of
/// [find_parameter_type](https://docs.rs/ethers/latest/ethers/?search=find_parameter_type)
/// in ethers-rs.
impl TryFrom<&str> for EthAbiParamType {
    type Error = EthError;

    fn try_from(iden: &str) -> Result<Self, Self::Error> {
        if let Some(param_type) = parse_param_type_array(iden) {
            return Ok(param_type);
        }
        if let Some(param_type) = parse_param_type_fixed_array(iden) {
            return Ok(param_type);
        }

        Ok(match iden.trim() {
            "address" => EthAbiParamType::Address,
            "bool" => EthAbiParamType::Bool,
            "bytes" => EthAbiParamType::Bytes,
            "h160" => EthAbiParamType::FixedBytes(20),
            "h256" | "secret" | "hash" => EthAbiParamType::FixedBytes(32),
            "h512" | "public" => EthAbiParamType::FixedBytes(64),
            "int256" | "int" | "uint" | "uint256" => EthAbiParamType::Uint(256),
            "string" => EthAbiParamType::String,
            typ => parse_param_type_integer(typ)
                .ok_or_else(|| Error::Other(format!("Invalid parameter type {typ}").into()))?,
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

impl From<&EthAbiToken> for Token {
    fn from(token: &EthAbiToken) -> Self {
        match token {
            EthAbiToken::Address(value) => Token::Address(*value),
            EthAbiToken::FixedBytes(value) => Token::FixedBytes(value.clone()),
            EthAbiToken::Bytes(value) => Token::Bytes(value.clone()),
            EthAbiToken::Int(value) => Token::Int(*value),
            EthAbiToken::Uint(value) => Token::Uint(*value),
            EthAbiToken::Bool(value) => Token::Bool(*value),
            EthAbiToken::String(value) => Token::String(value.clone()),
            EthAbiToken::FixedArray(values) => {
                Token::FixedArray(values.iter().map(Into::into).collect())
            }
            EthAbiToken::Array(values) => {
                Token::FixedArray(values.iter().map(Into::into).collect())
            }
            EthAbiToken::Tuple(values) => Token::Tuple(values.iter().map(Into::into).collect()),
        }
    }
}

/// Parse a string to parameter type Array, return None otherwise.
fn parse_param_type_array(iden: &str) -> Option<EthAbiParamType> {
    lazy_static! {
        // e.g. uint256[] or Person[]
        static ref ARRAY_REGEX: Regex = Regex::new(r"\A(.+)\[\]\z").unwrap();
    }

    let captures = ARRAY_REGEX.captures(iden.trim())?;
    let array_type: EthAbiParamType = captures.get(1)?.as_str().try_into().ok()?;

    Some(EthAbiParamType::Array(Box::new(array_type)))
}

/// Parse a string to parameter type FixedArray, return None otherwise.
fn parse_param_type_fixed_array(iden: &str) -> Option<EthAbiParamType> {
    lazy_static! {
        // e.g. uint256[100] or Person[100]
        static ref FIXED_ARRAY_REGEX: Regex = Regex::new(r"\A(.+)\[(\d+)\]\z").unwrap();
    }

    let captures = FIXED_ARRAY_REGEX.captures(iden.trim())?;
    let array_type: EthAbiParamType = captures.get(1)?.as_str().try_into().ok()?;
    let array_size = captures.get(2)?.as_str().parse::<usize>().ok()?;

    Some(EthAbiParamType::FixedArray(
        Box::new(array_type),
        array_size,
    ))
}

/// Parse a string to parameter type Int or Uint with specified size, return None otherwise.
fn parse_param_type_integer(iden: &str) -> Option<EthAbiParamType> {
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
