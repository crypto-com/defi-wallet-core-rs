use ethers::contract::ContractError;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::signer::SignerMiddlewareError;
use ethers::prelude::{
    abi, Http, ParseChainError, Provider, ProviderError, SignerMiddleware, Wallet,
};
use ethers::types::transaction::eip712;
use ethers::utils::ConversionError;

/// Possible errors from Ethereum transaction construction and broadcasting
#[derive(Debug, thiserror::Error)]
pub enum EthError {
    #[error("Converting from decimal failed")]
    DecConversion,
    #[error("Converting from hexadecimal failed")]
    HexConversion,
    #[error("Converting from decimal failed: {0}")]
    ParseError(ConversionError),
    #[error("Invalid node Web3 connection URL: {0}")]
    NodeUrl(url::ParseError),
    #[error("Transaction sending failed: {0}")]
    SendTxFail(SignerMiddlewareError<Provider<Http>, Wallet<SigningKey>>),
    #[error("Transaction sending failed: {0}")]
    BroadcastTxFail(ProviderError),
    #[error("Transaction dropped from the mempool")]
    MempoolDrop,
    #[error("Failed to obtain an account balance")]
    BalanceFail,
    #[error("Async Runtime error")]
    AsyncRuntimeError,
    #[error("Contract Send Error: {0}")]
    ContractSendError(ContractError<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>),
    #[error("Contract Call Error: {0}")]
    ContractCallError(ContractError<Provider<Http>>),
    #[error("Signature error")]
    SignatureError,
    #[error("Chainid error: {0}")]
    ChainidError(#[from] ParseChainError),
    #[error("ABI error: {0}")]
    AbiError(#[from] abi::Error),
    #[error("EIP-712 error: {0}")]
    Eip712Error(#[from] Eip712Error),
    #[error("Json parse error{0}")]
    JsonError(serde_json::Error),
}

/// EIP-712 related errors
#[derive(Debug, thiserror::Error)]
pub enum Eip712Error {
    #[error("Ethers error: {0}")]
    EthersError(#[from] eip712::Eip712Error),
    #[error("Invalid value of type {r#type}: {value}")]
    InvalidValueForType { r#type: String, value: String },
    #[error("Missing field: {0}")]
    MissingFieldError(String),
    #[error("Missing type: {0}")]
    MissingTypeError(String),
    #[error("SerdeJson error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Unsupported error: {0}")]
    UnsupportedError(String),
}
