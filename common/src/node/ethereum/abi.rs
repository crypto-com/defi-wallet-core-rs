use crate::EthError;
use ethers::prelude::abi::{ParamType, Token};
use ethers::prelude::{Address, H160, U256};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Ethereum ABI parameter type
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
                ParamType::Tuple(params.into_iter().map(Into::into).collect())
            }
        }
    }
}

/// Ethereum ABI token
#[derive(Serialize, Deserialize)]
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
                Token::FixedArray(values.into_iter().map(Into::into).collect())
            }
            EthAbiToken::Array(values) => {
                Token::FixedArray(values.into_iter().map(Into::into).collect())
            }
            EthAbiToken::Tuple(values) => {
                Token::Tuple(values.into_iter().map(Into::into).collect())
            }
        }
    }
}
