use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::{
    prelude::{Address, LocalWallet, Signer, TransactionRequest, U256},
    utils::{parse_units, ConversionError},
};
use std::{str::FromStr, sync::Arc};

use crate::SecretKey;

/// Possible errors from Ethereum transaction construction and broadcasting
#[derive(Debug, thiserror::Error)]
pub enum EthError {
    #[error("Converting from hexadecimal failed")]
    HexConversion,
    #[error("Converting from decimal failed: {0}")]
    ParseError(ConversionError),
    #[error("Invalid node Web3 connection URL")]
    NodeUrl,
    #[error("Transaction sending failed")]
    SendTxFail,
    #[error("Transaction dropped from the mempool")]
    MempoolDrop,
    #[error("Failed to obtain an account balance")]
    BalanceFail,
    #[error("Async Runtime error")]
    AsyncRuntimeError,
    #[error("Contract call error")]
    ContractError,
    #[error("Signature error")]
    SignatureError,
    #[error("Chainid error")]
    ChainidError,
}

/// Ethereum networks
/// TODO: to add more https://chainlist.org
pub enum EthNetwork {
    EthereumMainnet,
    RopstenTestnet,
    KovanTestnet,
    RinkebyTestnet,
    GoerliTestnet,
    Cronos,
    CronosTestnet,
    Custom { chain_id: u64 },
}

impl EthNetwork {
    /// return the corresponding chain-id as per https://chainlist.org
    pub fn get_chain_id(&self) -> u64 {
        match self {
            EthNetwork::EthereumMainnet => 1,
            EthNetwork::Cronos => 25,
            EthNetwork::CronosTestnet => 338,
            EthNetwork::Custom { chain_id } => *chain_id,
            EthNetwork::RopstenTestnet => 3,
            EthNetwork::KovanTestnet => 42,
            EthNetwork::RinkebyTestnet => 4,
            EthNetwork::GoerliTestnet => 5,
        }
    }
}

/// The gas/native token amount
/// in decimal notation
pub enum EthAmount {
    /// 10^-18 ETH
    WeiDecimal {
        amount: String,
    },
    /// 10^-9 ETH
    GweiDecimal {
        amount: String,
    },
    EthDecimal {
        amount: String,
    },
}

impl TryInto<U256> for EthAmount {
    type Error = ConversionError;

    fn try_into(self) -> Result<U256, Self::Error> {
        match self {
            EthAmount::WeiDecimal { amount } => parse_units(amount, "wei"),
            EthAmount::GweiDecimal { amount } => parse_units(amount, "gwei"),
            EthAmount::EthDecimal { amount } => parse_units(amount, "ether"),
        }
    }
}

/// constructs a simple transfer of Eth/native token on a given network
pub fn construct_simple_eth_transfer_tx(
    to_hex: &str,
    amount: EthAmount,
) -> Result<TransactionRequest, EthError> {
    let to = Address::from_str(to_hex).map_err(|_| EthError::HexConversion)?;
    let amount: U256 = amount.try_into().map_err(EthError::ParseError)?;
    Ok(TransactionRequest::pay(to, amount))
}

/// constructs an unsigned simple transfer of Eth/native token on a given network
pub fn construct_unsigned_eth_tx(
    from_hex: &str,
    to_hex: &str,
    amount: EthAmount,
    network: EthNetwork,
) -> Result<Vec<u8>, EthError> {
    let from = Address::from_str(from_hex).map_err(|_| EthError::HexConversion)?;

    let tx = construct_simple_eth_transfer_tx(to_hex, amount)?;
    Ok(tx
        .from(from)
        .chain_id(network.get_chain_id())
        .rlp()
        .to_vec())
}

/// constructs a signed simple transfer of Eth/native token on a given network
pub fn build_signed_eth_tx(
    to_hex: &str,
    amount: EthAmount,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
) -> Result<Vec<u8>, EthError> {
    let tx = construct_simple_eth_transfer_tx(to_hex, amount)?;
    let wallet =
        LocalWallet::from(secret_key.get_signing_key()).with_chain_id(network.get_chain_id());
    let typed_tx = TypedTransaction::Legacy(tx.clone());

    let sig = wallet.sign_transaction_sync(&typed_tx);
    let signed_tx = &tx.rlp_signed(&sig);
    Ok(signed_tx.to_vec())
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use ethers::utils::rlp::Rlp;

    use crate::*;

    #[test]
    fn eth_tx_works() {
        let tx_raw = construct_unsigned_eth_tx(
            "0x2c600e0a72b3ae39e9b27d2e310b180abe779368",
            "0x2c600e0a72b3ae39e9b27d2e310b180abe779368",
            EthAmount::EthDecimal {
                amount: "1".to_string(),
            },
            EthNetwork::Cronos,
        )
        .expect("ok signed tx");
        assert!(Rlp::new(&tx_raw).payload_info().is_ok());
    }

    #[test]
    fn eth_signing_works() {
        let secret_key = SecretKey::new();

        let tx_raw = build_signed_eth_tx(
            "0x2c600e0a72b3ae39e9b27d2e310b180abe779368",
            EthAmount::EthDecimal {
                amount: "1".to_string(),
            },
            EthNetwork::Cronos,
            Arc::new(secret_key),
        )
        .expect("ok signed tx");
        assert!(Rlp::new(&tx_raw).payload_info().is_ok());
    }
}
