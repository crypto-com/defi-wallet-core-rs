use crate::{SecretKey, WalletCoin, WalletCoinFunc};
use ethers::prelude::{
    Address, Chain, Eip1559TransactionRequest, Eip2930TransactionRequest, LocalWallet, Signer,
    TransactionRequest, U256,
};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::transaction::eip2930::AccessList;
use ethers::types::{Bytes, NameOrAddress, U64};
use ethers::utils::{parse_units, ConversionError};
use serde::Deserialize;
use std::default::Default;
use std::str::FromStr;
use std::sync::Arc;
mod abi_contract;
mod error;
mod signer;
use ethers::types::Signature;

#[cfg(feature = "abi-contract")]
pub use abi_contract::*;
pub use error::*;
#[cfg(feature = "abi-contract")]
pub use signer::*;

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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    let to = Address::from_str(to_hex).map_err(|_| EthError::HexConversion);
    let amount: U256 = amount.try_into().map_err(EthError::ParseError)?;
    if legacy_tx {
        let mut txrequest = TransactionRequest::new();
        if let Ok(tovalue) = to {
            txrequest = txrequest.to::<NameOrAddress>(tovalue.into());
        };
        let typedtx = txrequest.value(amount).from(from).chain_id(chain_id).into();
        Ok(typedtx)
    } else {
        let mut txrequest = Eip1559TransactionRequest::new();
        if let Ok(tovalue) = to {
            txrequest = txrequest.to::<NameOrAddress>(tovalue.into());
        }
        let typedtx = txrequest.value(amount).from(from).chain_id(chain_id).into();
        Ok(typedtx)
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
    tx.set_nonce(U256::from_dec_str(&tx_info.nonce).map_err(EthError::DecConversion)?);
    tx.set_gas(U256::from_dec_str(&tx_info.gas_limit).map_err(EthError::DecConversion)?);
    let gas_price: U256 = tx_info.gas_price.try_into().map_err(EthError::ParseError)?;
    tx.set_gas_price(gas_price);
    if let Some(data) = tx_info.data {
        tx.set_data(data.into());
    }
    tx.set_chain_id(chain_id);
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let sig: Signature = wallet.sign_transaction_sync(&tx);
    let signed_tx = &tx.rlp_signed(&sig);
    Ok(signed_tx.to_vec())
}

/// builds an signed ethereum transaction given the inputs and signature
pub fn build_signed_eth_tx_with_signature(
    tx_info: EthTxInfo,
    network: EthNetwork,
    from_address: &str,
    signature: &Signature, // 65 bytes
) -> Result<Vec<u8>, EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;
    let mut tx: TypedTransaction = construct_simple_eth_transfer_tx(
        from_address,
        &tx_info.to_address,
        tx_info.amount,
        tx_info.legacy_tx || legacy,
        chain_id,
    )?;
    tx.set_nonce(U256::from_dec_str(&tx_info.nonce).map_err(EthError::DecConversion)?);
    tx.set_gas(U256::from_dec_str(&tx_info.gas_limit).map_err(EthError::DecConversion)?);
    let gas_price: U256 = tx_info.gas_price.try_into().map_err(EthError::ParseError)?;
    tx.set_gas_price(gas_price);
    if let Some(data) = tx_info.data {
        tx.set_data(data.into());
    }
    let signed_tx = &tx.rlp_signed(signature);
    Ok(signed_tx.to_vec())
}

/// Parameters for sending a transaction
#[derive(Clone, Default, Deserialize, PartialEq, Eq, Debug)]
pub struct DynamicTransactionRequest {
    /// Sender address or ENS name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,

    /// Recipient address (None for contract creation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<NameOrAddress>,

    /// Supplied gas (None for sensible default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,

    /// Gas price (None for sensible default)
    #[serde(rename = "gasPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,

    /// Transfered value (None for no transfer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,

    /// The compiled code of a contract OR the first 4 bytes of the hash of the
    /// invoked method signature and encoded parameters. For details see Ethereum Contract ABI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,

    /// Transaction nonce (None for next available nonce)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,

    /// Accessed List (contains address and storageKeys)
    #[serde(rename = "accessList", default)]
    pub access_list: AccessList,

    /// Represents the maximum tx fee that will go to the miner as part of the user's
    /// fee payment. It serves 3 purposes:
    /// 1. Compensates miners for the uncle/ommer risk + fixed costs of including transaction in a
    /// block; 2. Allows users with high opportunity costs to pay a premium to miners;
    /// 3. In times where demand exceeds the available block space (i.e. 100% full, 30mm gas),
    /// this component allows first price auctions (i.e. the pre-1559 fee model) to happen on the
    /// priority fee.
    ///
    /// More context [here](https://hackmd.io/@q8X_WM2nTfu6nuvAzqXiTQ/1559-wallets)
    #[serde(
        rename = "maxPriorityFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_priority_fee_per_gas: Option<U256>,

    /// Represents the maximum amount that a user is willing to pay for their tx (inclusive of
    /// baseFeePerGas and maxPriorityFeePerGas). The difference between maxFeePerGas and
    /// baseFeePerGas + maxPriorityFeePerGas is “refunded” to the user.
    #[serde(
        rename = "maxFeePerGas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_fee_per_gas: Option<U256>,

    #[serde(skip_serializing)]
    #[serde(default, rename = "chainId")]
    /// Chain ID (None for mainnet)
    pub chain_id: Option<U64>,
}

/// Dynamic structure for multiple transactions
impl DynamicTransactionRequest {
    /// Convert to Eip 1559 Transaction Request
    pub fn to_eip1559_tx(self) -> Eip1559TransactionRequest {
        let mut tx = Eip1559TransactionRequest::new().access_list(self.access_list);
        tx.chain_id = self.chain_id;
        tx.data = self.data;
        tx.from = self.from;
        tx.to = self.to;
        tx.gas = self.gas;
        tx.max_fee_per_gas = self.max_fee_per_gas;
        tx.max_priority_fee_per_gas = self.max_priority_fee_per_gas;
        tx.nonce = self.nonce;
        tx.value = self.value;
        tx
    }

    /// Convert to legacy Transaction Request
    pub fn to_legacy_tx(self) -> TransactionRequest {
        let mut tx = TransactionRequest::new();
        tx.chain_id = self.chain_id;
        tx.data = self.data;
        tx.from = self.from;
        tx.to = self.to;
        tx.gas = self.gas;
        tx.gas_price = self.gas_price;
        tx.nonce = self.nonce;
        tx.value = self.value;
        tx
    }

    /// Convert to Eip 2930 Transaction Request
    pub fn to_eip2930_tx(self) -> Eip2930TransactionRequest {
        Eip2930TransactionRequest::new(self.clone().to_legacy_tx(), self.access_list)
    }

    /// Convert to TypedTransaction
    pub fn to_type_tx(self) -> TypedTransaction {
        if self.max_fee_per_gas.is_some() {
            TypedTransaction::Eip1559(self.to_eip1559_tx())
        } else if !self.access_list.0.is_empty() {
            TypedTransaction::Eip2930(self.to_eip2930_tx())
        } else {
            TypedTransaction::Legacy(self.to_legacy_tx())
        }
    }
}

/// Parse the json data that meets the walletconnect standard and build raw transaction
/// Use the chainid specified in json, if not set, use the default chainid, its value is 1
pub fn eth_sign_transaction(
    json_str: &str,
    secret_key: Arc<SecretKey>,
) -> Result<Vec<u8>, EthError> {
    let mut default_chain_id: u64 = 1;
    let tx: DynamicTransactionRequest =
        serde_json::from_str(json_str).map_err(EthError::JsonError)?;
    let type_tx: TypedTransaction = tx.to_type_tx();
    if type_tx.chain_id().is_some() {
        default_chain_id = type_tx.chain_id().unwrap().as_u64();
    }
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(default_chain_id);
    let sig = wallet.sign_transaction_sync(&type_tx);
    let signed_tx = &type_tx.rlp_signed(&sig);
    Ok(signed_tx.to_vec())
}

/// Parse the json data that meets the walletconnect standard and build raw transaction
/// Sign with the specified chainid
pub fn eth_sign_transaction_with_chainid(
    json_str: &str,
    secret_key: Arc<SecretKey>,
    chain_id: u64,
) -> Result<Vec<u8>, EthError> {
    let mut tx: DynamicTransactionRequest =
        serde_json::from_str(json_str).map_err(EthError::JsonError)?;
    tx.chain_id = Some(chain_id.into());
    let type_tx: TypedTransaction = tx.to_type_tx();

    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let sig = wallet.sign_transaction_sync(&type_tx);
    let signed_tx = &type_tx.rlp_signed(&sig);
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
    fn eip1559_tx_test() {
        let words = "lumber flower voice hood obvious behave relax chief warm they they mountain";

        let wallet = HDWallet::recover_wallet(words.to_owned(), Some("".to_owned()))
            .expect("Failed to recover wallet");
        let secret_key = wallet
            .get_key_from_index(
                WalletCoin::Ethereum {
                    network: EthNetwork::Mainnet,
                },
                1,
            )
            .expect("get_key_from_index error");

        let json_str = r#"{"from":"0x68418d0fdb846e8736aa613159035a9d9fde11f0","to":"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d","gas":"0x5208","value":"0xde0b6b3a7640000","data":"0x","nonce":"0x0","maxPriorityFeePerGas":"0x1","maxFeePerGas":"0x77359401","chainId":"0x0539"}"#;
        let tx_raw = eth_sign_transaction_with_chainid(json_str, secret_key.clone(), 1337).unwrap();
        assert_eq!(hex::encode(tx_raw),"02f87082053980018477359401825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080c001a0caa0df6665a08e4fae0839395387aabeeef4064134dd09a771eed6e41d6c258da07817000d01107a554e8e885c872a672df50e2dc25ed5068b83a93e2a27982bce");

        let json_str = r#"{"from":"0x68418d0fdb846e8736aa613159035a9d9fde11f0","to":"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d","gas":"0x5208","value":"0xde0b6b3a7640000","data":"0x","nonce":"0x0","maxPriorityFeePerGas":"0x1","maxFeePerGas":"0x77359401","chainId":"0x0539"}"#;
        let tx_raw = eth_sign_transaction(json_str, secret_key.clone()).unwrap();
        assert_eq!(hex::encode(tx_raw),"02f87082053980018477359401825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080c001a0caa0df6665a08e4fae0839395387aabeeef4064134dd09a771eed6e41d6c258da07817000d01107a554e8e885c872a672df50e2dc25ed5068b83a93e2a27982bce");

        let json_str = r#"{"from":"0x68418d0fdb846e8736aa613159035a9d9fde11f0","to":"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d","gas":"0x5208","value":"0xde0b6b3a7640000","data":"0x","nonce":"0x0","accessList":[{"address":"0x0000000000000000000000000000000000000000","storageKeys":["0x0000000000000000000000000000000000000000000000000000000000000000"]}],"maxPriorityFeePerGas":"0x1","maxFeePerGas":"0x77359401","chainId":"0x0539"}"#;
        let tx_raw = eth_sign_transaction(json_str, secret_key).unwrap();
        assert_eq!(hex::encode(tx_raw),"02f8a982053980018477359401825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080f838f7940000000000000000000000000000000000000000e1a0000000000000000000000000000000000000000000000000000000000000000080a0462c27c0ae0a8a2fd448ab299d519823c7016c280881c38747dcda913dc1c4caa0685acccb1f37f87250e9688e805725f2eb0e9f63b53fe311f9ed485f07987cf4");
    }

    #[test]
    fn eip2930_tx_test() {
        let words = "lumber flower voice hood obvious behave relax chief warm they they mountain";

        let wallet = HDWallet::recover_wallet(words.to_owned(), Some("".to_owned()))
            .expect("Failed to recover wallet");
        let secret_key = wallet
            .get_key_from_index(
                WalletCoin::Ethereum {
                    network: EthNetwork::Mainnet,
                },
                1,
            )
            .expect("get_key_from_index error");

        let json_str = r#"{"from":"0x68418d0fdb846e8736aa613159035a9d9fde11f0","to":"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d","gas":"0x5208","gasPrice":"0x5f5e100","value":"0xde0b6b3a7640000","data":"0x","nonce":"0x0","accessList":[{"address":"0x0000000000000000000000000000000000000000","storageKeys":["0x0000000000000000000000000000000000000000000000000000000000000000"]}],"chainId":"0x0539"}"#;
        let tx_raw = eth_sign_transaction_with_chainid(json_str, secret_key, 1337).unwrap();
        assert_eq!(hex::encode(tx_raw),"01f8a8820539808405f5e100825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080f838f7940000000000000000000000000000000000000000e1a0000000000000000000000000000000000000000000000000000000000000000080a024117c04934ced6c3d272447816f0ebc00e97dd012f8d3872d661a48152c0e5ca0601c21637bad2f399da6a7e314a6119956f4bb8c2d7dd2df6905786e56a35c47");
    }

    #[test]
    fn legacy_tx_test() {
        let words = "lumber flower voice hood obvious behave relax chief warm they they mountain";

        let wallet = HDWallet::recover_wallet(words.to_owned(), Some("".to_owned()))
            .expect("Failed to recover wallet");
        let secret_key = wallet
            .get_key_from_index(
                WalletCoin::Ethereum {
                    network: EthNetwork::Mainnet,
                },
                1,
            )
            .expect("get_key_from_index error");

        let json_str = r#"{"from":"0x68418d0fdb846e8736aa613159035a9d9fde11f0","to":"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d","gas":"0x5208","gasPrice":"0x5f5e100","value":"0xde0b6b3a7640000","data":"0x","nonce":"0x0","chainId":"0x0539"}"#;
        let tx_raw = eth_sign_transaction_with_chainid(json_str, secret_key, 1337).unwrap();
        assert_eq!(hex::encode(tx_raw),"f86d808405f5e100825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080820a96a0dd110c3396ac52d7a23db8e5cca23b42983636192190baeec2178d5b33b02369a057ace20b2e326e7e24b0e1ca57d312b19a29a8353301d2280e5a829fa7866f10");
    }

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
            WalletCoinFunc {
                coin: WalletCoin::Ethereum {
                    network: EthNetwork::Polygon,
                },
            }
            .get_eth_network(),
            secret_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "f86b808203e8825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080820135a01c41699ee874ae206cc364c60ad699a840085ecd72a3c700cf9cae84cefc2373a056dacb5e4a89073ab83f93c6e4ed706019ec68f569d1930c6e29272bd9361525",
        );
    }
}
