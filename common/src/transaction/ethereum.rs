use crate::{SecretKey, WalletCoin, WalletCoinFunc};
use ethers::contract::ContractError;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::signer::SignerMiddlewareError;
use ethers::prelude::{
    abi, Address, Chain, Eip1559TransactionRequest, Http, LocalWallet, ParseChainError, Provider,
    ProviderError, Signer, SignerMiddleware, TransactionRequest, Wallet, U256,
};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::transaction::eip712::Eip712Error;
use ethers::utils::{parse_units, ConversionError};
use std::default::Default;
use std::str::FromStr;
use std::sync::Arc;

mod abi_contract;

#[cfg(feature = "abi-contract")]
pub use abi_contract::*;

/// Possible errors from Ethereum transaction construction and broadcasting
#[derive(Debug, thiserror::Error)]
pub enum EthError {
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
    ChainidError(ParseChainError),
    #[error("ABI error: {0}")]
    AbiError(abi::Error),
    #[error("EIP-712 error: {0}")]
    Eip712Error(Eip712Error),
    #[error("Common error: {0}")]
    CommonError(String),
}

impl From<abi::Error> for EthError {
    fn from(abi_error: abi::Error) -> EthError {
        EthError::AbiError(abi_error)
    }
}

impl From<Eip712Error> for EthError {
    fn from(eip712_error: Eip712Error) -> EthError {
        EthError::Eip712Error(eip712_error)
    }
}

impl From<ParseChainError> for EthError {
    fn from(parse_chain_error: ParseChainError) -> EthError {
        EthError::ChainidError(parse_chain_error)
    }
}

/// Ethereum networks
/// the string conversion is from: https://github.com/gakonst/ethers-rs/blob/4fd9c7800ee9afd5395d8c7b8652d788b9e80f35/ethers-core/src/types/chain.rs#L130
/// e.g. "mainnet" == Ethereum mainnet
#[derive(Clone)]
pub enum EthNetwork {
    /// Ethereum mainnet
    Mainnet,
    /// Binance smart chain
    BSC,
    /// Cronos
    Cronos,
    /// Polygon
    Polygon,
    /// Known network with specified name
    Known { name: String },
    /// Custom network with chain ID and legacy flag
    Custom { chain_id: u64, legacy: bool },
}

impl EthNetwork {
    /// returns the chain id and if the chain needs the legacy
    /// transaction request
    pub fn to_chain_params(self) -> Result<(u64, bool), EthError> {
        match self {
            EthNetwork::Custom { chain_id, legacy } => Ok((chain_id, legacy)),
            _ => {
                let chain = Chain::try_from(self)?;
                Ok((chain.into(), chain.is_legacy()))
            }
        }
    }
}

impl Default for EthNetwork {
    fn default() -> Self {
        EthNetwork::Mainnet
    }
}

impl TryFrom<EthNetwork> for Chain {
    type Error = EthError;

    fn try_from(network: EthNetwork) -> Result<Chain, Self::Error> {
        Ok(match network {
            EthNetwork::Mainnet => Chain::Mainnet,
            EthNetwork::BSC => Chain::BinanceSmartChain,
            EthNetwork::Cronos => Chain::Cronos,
            EthNetwork::Polygon => Chain::Polygon,
            EthNetwork::Known { name } => Chain::from_str(&name)?,
            EthNetwork::Custom { chain_id, .. } => Chain::try_from(chain_id)?,
        })
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
    from_hex: &str,
    to_hex: &str,
    amount: EthAmount,
    legacy_tx: bool,
    chain_id: u64,
) -> Result<TypedTransaction, EthError> {
    let from = Address::from_str(from_hex).map_err(|_| EthError::HexConversion)?;
    let to = Address::from_str(to_hex).map_err(|_| EthError::HexConversion)?;
    let amount: U256 = amount.try_into().map_err(EthError::ParseError)?;
    if legacy_tx {
        Ok(TransactionRequest::pay(to, amount)
            .from(from)
            .chain_id(chain_id)
            .into())
    } else {
        Ok(Eip1559TransactionRequest::new()
            .to(to)
            .value(amount)
            .from(from)
            .chain_id(chain_id)
            .into())
    }
}

/// constructs an unsigned simple transfer of Eth/native token on a given network
pub fn construct_unsigned_eth_tx(
    from_hex: &str,
    to_hex: &str,
    amount: EthAmount,
    network: EthNetwork,
    legacy_tx: bool,
) -> Result<Vec<u8>, EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;

    let tx =
        construct_simple_eth_transfer_tx(from_hex, to_hex, amount, legacy || legacy_tx, chain_id)?;
    Ok(tx.rlp().to_vec())
}

/// A common information for ethereum transactions
pub struct EthTxInfo {
    /// the destination address as a hexadecimal string
    pub to_address: String,
    /// the amount to send
    pub amount: EthAmount,
    /// the nonce as a decimal string
    pub nonce: String,
    /// the gas limit as a decimal string
    pub gas_limit: String,
    /// the gas price to pay
    pub gas_price: EthAmount,
    /// optional data
    pub data: Option<Vec<u8>>,
    /// use the legacy tx format (even if the chain supports EIP-1559)
    pub legacy_tx: bool,
}

/// builds a signed ethereum transaction given the inputs
pub fn build_signed_eth_tx(
    tx_info: EthTxInfo,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
) -> Result<Vec<u8>, EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;
    let from_address = WalletCoinFunc {
        coin: WalletCoin::Ethereum {
            network: EthNetwork::Mainnet,
        },
    }
    .derive_address(secret_key.as_ref())
    .map_err(|_| EthError::HexConversion)?;
    let mut tx: TypedTransaction = construct_simple_eth_transfer_tx(
        &from_address,
        &tx_info.to_address,
        tx_info.amount,
        tx_info.legacy_tx || legacy,
        chain_id,
    )?;
    tx.set_nonce(
        U256::from_dec_str(&tx_info.nonce)
            .map_err(|e| EthError::ParseError(ConversionError::FromDecStrError(e)))?,
    );
    tx.set_gas(
        U256::from_dec_str(&tx_info.gas_limit)
            .map_err(|e| EthError::ParseError(ConversionError::FromDecStrError(e)))?,
    );
    let gas_price: U256 = tx_info.gas_price.try_into().map_err(EthError::ParseError)?;
    tx.set_gas_price(gas_price);
    if let Some(data) = tx_info.data {
        tx.set_data(data.into());
    }
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let sig = wallet.sign_transaction_sync(&tx);
    let signed_tx = &tx.rlp_signed(&sig);
    Ok(signed_tx.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HDWallet;
    use crate::{SecretKey, WalletCoin, WalletCoinFunc};
    use ethers::utils::hex;
    use ethers::utils::rlp::Rlp;
    use std::sync::Arc;

    #[test]
    fn eth_tx_works() {
        let tx_raw = construct_unsigned_eth_tx(
            "0x2c600e0a72b3ae39e9b27d2e310b180abe779368",
            "0x2c600e0a72b3ae39e9b27d2e310b180abe779368",
            EthAmount::EthDecimal {
                amount: "1".to_string(),
            },
            EthNetwork::Known {
                name: "cronos".into(),
            },
            false,
        )
        .expect("ok signed tx");
        assert!(Rlp::new(&tx_raw).payload_info().is_ok());
    }

    #[test]
    fn eth_signing_works() {
        let secret_key = SecretKey::new();
        let tx_info = EthTxInfo {
            to_address: "0x2c600e0a72b3ae39e9b27d2e310b180abe779368".to_string(),
            amount: EthAmount::EthDecimal {
                amount: "1".to_string(),
            },
            nonce: "0".to_string(),
            gas_limit: "21000".to_string(),
            gas_price: EthAmount::WeiDecimal {
                amount: "7".to_string(),
            },
            data: Some(vec![]),
            legacy_tx: false,
        };
        let tx_raw = build_signed_eth_tx(
            tx_info,
            EthNetwork::Known {
                name: "cronos".into(),
            },
            Arc::new(secret_key),
        )
        .expect("ok signed tx");
        assert!(Rlp::new(&tx_raw).payload_info().is_ok());
    }

    #[test]
    fn eth_tx_test() {
        // check normal tx
        let hex = "24e585759e492f5e810607c82c202476c22c5876b10247ebf8b2bb7f75dbed2e";
        let secret_key =
            SecretKey::from_hex(hex.to_owned()).expect("Failed to construct Secret Key from hex");
        println!(
            "{}",
            secret_key
                .to_address(WalletCoin::Ethereum {
                    network: EthNetwork::Mainnet
                })
                .expect("address error")
        );
        let tx_info = EthTxInfo {
            to_address: "0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d".to_string(),
            amount: EthAmount::EthDecimal {
                amount: "1".to_string(),
            },
            nonce: "0".to_string(),
            gas_limit: "21000".to_string(),
            gas_price: EthAmount::WeiDecimal {
                amount: "1000".to_string(),
            },
            data: Some(vec![]),
            legacy_tx: true,
        };

        let tx_raw = build_signed_eth_tx(
            tx_info,
            EthNetwork::Custom {
                chain_id: 1,
                legacy: true,
            },
            Arc::new(secret_key),
        )
        .expect("ok signed tx");
        assert_eq!(
            hex::encode(tx_raw),
            "f869808203e8825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a76400008026a0f65f41ceaadda3c64f68c4d65b202b89a8dc508bbd0957ba28c61eb65ba694f6a03d5c681c4a5c21f4ad1616aed9a0e0b72344dbcfdeddb60a11bfc19a11e60120",
        );
    }

    #[test]
    fn polygon_tx_test() {
        let words = "lumber flower voice hood obvious behave relax chief warm they they mountain";

        let wallet = HDWallet::recover_wallet(words.to_owned(), Some("".to_owned()))
            .expect("Failed to recover wallet");
        let secret_key = wallet
            .get_key_from_index(
                WalletCoin::Ethereum {
                    network: EthNetwork::Polygon,
                },
                1,
            )
            .expect("get_key_from_index error");

        let (_, legacy) = WalletCoinFunc {
            coin: WalletCoin::Ethereum {
                network: EthNetwork::Polygon,
            },
        }
        .get_eth_network()
        .to_chain_params()
        .expect("");

        let tx_info = EthTxInfo {
            to_address: "0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d".to_string(),
            amount: EthAmount::EthDecimal {
                amount: "1".to_string(),
            },
            nonce: "0".to_string(),
            gas_limit: "21000".to_string(),
            gas_price: EthAmount::WeiDecimal {
                amount: "1000".to_string(),
            },
            data: Some(vec![]),
            legacy_tx: legacy,
        };

        let tx_raw = build_signed_eth_tx(
            tx_info,
            WalletCoinFunc {
                coin: WalletCoin::Ethereum {
                    network: EthNetwork::Polygon,
                },
            }
            .get_eth_network(),
            secret_key,
        )
        .expect("ok signed tx");
        println!("{}", hex::encode(tx_raw))
        // assert_eq!(
        //     hex::encode(tx_raw),
        //     "f86b808203e8825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080820135a01c41699ee874ae206cc364c60ad699a840085ecd72a3c700cf9cae84cefc2373a056dacb5e4a89073ab83f93c6e4ed706019ec68f569d1930c6e29272bd9361525",
        // );
    }
}
