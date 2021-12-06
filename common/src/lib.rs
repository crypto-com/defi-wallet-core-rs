pub use cosmrs::*;
uniffi_macros::include_scaffolding!("common");
use wasm_bindgen::prelude::*;
mod utils;
use cosmrs::{
    bank::MsgSend,
    bip32::{PublicKey, PublicKeyBytes, KEY_SIZE},
    crypto::{self, secp256k1::VerifyingKey},
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
};
use eyre::{eyre, Context};

/// human-readable bech32 prefix for Crypto.org Chain accounts
pub const CRYPTO_ORG_BECH32_HRP: &str = "cro";
/// human-readable bech32 prefix for Crypto.org Chain testnet accounts
pub const CRYPTO_ORG_TESTNET_BECH32_HRP: &str = "tcro";
/// human-readable bech32 prefix for Cronos accounts
pub const CRONOS_BECH32_HRP: &str = "crc";
/// human-readable bech32 prefix for Cosmos Hub accounts
pub const COSMOS_BECH32_HRP: &str = "cosmos";
/// mainnet chain id of Crypto.org Chain
pub const CRYPTO_ORG_CHAIN_ID: &str = "crypto-org-chain-mainnet-1";
/// testnet chain id of Crypto.org Chain Croeseid
pub const CRYPTO_ORG_CHAIN_TESTNET_ID: &str = "testnet-croeseid-4";
/// mainnet chain id of Cronos
pub const CRONOS_CHAIN_ID: &str = "cronosmainnet_25-1";
/// mainnet chain id of Cosmos Hub
pub const COSMOS_CHAIN_ID: &str = "cosmoshub-4";

/// Network to work with
pub enum Network {
    /// Crypto.org Chain mainnet
    CryptoOrgMainnet,
    /// Crypto.org Chain testnet
    CryptoOrgTestnet,
    /// Cronos mainnet beta
    CronosMainnet,
    /// Cosmos Hub mainnet
    CosmosHub,
    /// other network
    Other {
        /// Tendermint Chain Id
        chain_id: String,
        /// HD wallet coin type
        coin_type: u32,
        /// bech32 human-readable prefix
        bech32hrp: String,
    },
}

impl Network {
    fn get_bech32_hrp(&self) -> &str {
        match self {
            Network::CryptoOrgMainnet => CRYPTO_ORG_BECH32_HRP,
            Network::CronosMainnet => CRONOS_BECH32_HRP,
            Network::CosmosHub => COSMOS_BECH32_HRP,
            Network::Other { bech32hrp, .. } => bech32hrp,
            Network::CryptoOrgTestnet => CRYPTO_ORG_TESTNET_BECH32_HRP,
        }
    }

    fn get_chain_id(&self) -> eyre::Result<tendermint::chain::Id> {
        let chain_id = match self {
            Network::CryptoOrgMainnet => CRYPTO_ORG_CHAIN_ID,
            Network::CronosMainnet => CRONOS_CHAIN_ID,
            Network::CosmosHub => COSMOS_CHAIN_ID,
            Network::Other { chain_id, .. } => chain_id,
            Network::CryptoOrgTestnet => CRYPTO_ORG_CHAIN_TESTNET_ID,
        };
        chain_id.parse().context("invalid chain id")
    }
}

/// single coin amount
pub enum SingleCoin {
    /// basecro
    BaseCRO { amount: u64 },
    /// 1 CRO = 10^8 basecro on Crypto.org Chain mainnet OR 10^18 on Cronos/EVM
    CRO { amount: u64, network: Network },
    /// basecro
    TestnetBaseCRO { amount: u64 },
    /// 1 TCRO = 10^8 basetcro
    TestnetCRO { amount: u64 },
    /// uatom
    UATOM { amount: u64 },
    /// 1 ATOM = 10^6 uatom
    ATOM { amount: u64 },
    /// other coin unit
    Other { amount: String, denom: String },
}

impl TryInto<Coin> for &SingleCoin {
    type Error = ErrorReport;

    fn try_into(self) -> eyre::Result<Coin> {
        match self {
            SingleCoin::BaseCRO { amount } => Ok(Coin {
                amount: (*amount).into(),
                denom: "basecro".parse()?,
            }),
            SingleCoin::TestnetBaseCRO { amount } => Ok(Coin {
                amount: (*amount).into(),
                denom: "basetcro".parse()?,
            }),
            SingleCoin::TestnetCRO { amount } => {
                let base_amount = amount
                    .checked_mul(10 ^ 8)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(Coin {
                    amount: base_amount.into(),
                    denom: "basetcro".parse()?,
                })
            }
            SingleCoin::CRO { amount, network } => {
                let decimals = match network {
                    Network::CronosMainnet => 10 ^ 18,
                    _ => 10 ^ 8,
                };
                // FIXME: convert to Decimal when it supports multiplication
                let base_amount = amount
                    .checked_mul(decimals)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(Coin {
                    amount: base_amount.into(),
                    denom: "basecro".parse()?,
                })
            }
            SingleCoin::UATOM { amount } => Ok(Coin {
                amount: (*amount).into(),
                denom: "uatom".parse()?,
            }),
            SingleCoin::ATOM { amount } => {
                let base_amount = amount
                    .checked_mul(1_000_000)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(Coin {
                    amount: base_amount.into(),
                    denom: "uatom".parse()?,
                })
            }
            SingleCoin::Other { amount, denom } => Ok(Coin {
                amount: amount.parse()?,
                denom: denom.parse()?,
            }),
        }
    }
}

/// wrapper around 33-byte secp256k1 public key
/// FIXME: investigate wrapping directly `cosmrs::crypto::PublicKey`
pub struct PublicKeyBytesWrapper(Vec<u8>);

/// unwrapping public key errors
/// FIXME: additional errors after wrapping directly `cosmrs::crypto::PublicKey`
#[derive(Debug, thiserror::Error)]
pub enum PublicKeyBytesError {
    #[error("The length should be 33-bytes")]
    InvalidLength,
}

const COMPRESSED_SECP256K1_PUBKEY_SIZE: usize = KEY_SIZE + 1;

impl From<PublicKeyBytesWrapper> for PublicKeyBytes {
    fn from(wrapper: PublicKeyBytesWrapper) -> Self {
        let mut result = [0u8; COMPRESSED_SECP256K1_PUBKEY_SIZE];
        result.copy_from_slice(&wrapper.0);
        result
    }
}

impl UniffiCustomTypeWrapper for PublicKeyBytesWrapper {
    type Wrapped = Vec<u8>;

    fn wrap(val: Self::Wrapped) -> uniffi::Result<Self> {
        if val.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
            Err(PublicKeyBytesError::InvalidLength.into())
        } else {
            Ok(PublicKeyBytesWrapper(val))
        }
    }

    fn unwrap(obj: Self) -> Self::Wrapped {
        obj.0
    }
}

/// the common transaction data needed for Cosmos SDK transactions
#[wasm_bindgen]
pub struct CosmosSDKTxInfo {
    /// 33-byte secp256k1 public of the sender
    sender_pubkey: PublicKeyBytesWrapper,
    /// global account number of the sender
    account_number: u64,
    /// equivalent of "account nonce"
    sequence_number: u64,
    /// the maximum gas limit
    gas_limit: u64,
    /// the fee to be paid (gas_limit * gas_price)
    fee_amount: SingleCoin,
    /// transaction timeout
    timeout_height: u32,
    /// optional memo
    memo_note: Option<String>,
    /// the network to use
    network: Network,
}

/// Cosmos SDK message types
pub enum CosmosSDKMsg {
    /// MsgSend
    BankSend {
        /// recipient address in bech32
        recipient_address: String,
        /// amount to send
        amount: SingleCoin,
    },
}

impl CosmosSDKMsg {
    fn to_any(&self, sender_address: AccountId) -> eyre::Result<Any> {
        match self {
            CosmosSDKMsg::BankSend {
                recipient_address,
                amount,
            } => {
                let amount_coin: Coin = amount.try_into()?;
                let recipient_account_id = recipient_address.parse::<AccountId>()?;
                let msg_send = MsgSend {
                    from_address: sender_address,
                    to_address: recipient_account_id,
                    amount: vec![amount_coin],
                };
                msg_send.to_any()
            }
        }
    }
}

fn get_single_msg_signdoc(tx_info: CosmosSDKTxInfo, msg: CosmosSDKMsg) -> eyre::Result<SignDoc> {
    let chain_id = tx_info.network.get_chain_id()?;
    let sender_public_key: crypto::PublicKey =
        crypto::PublicKey::from(VerifyingKey::from_bytes(tx_info.sender_pubkey.into())?);
    let sender_account_id = sender_public_key.account_id(tx_info.network.get_bech32_hrp())?;

    let tx_body = tx::Body::new(
        vec![msg.to_any(sender_account_id)?],
        tx_info.memo_note.unwrap_or_default(),
        tx_info.timeout_height,
    );
    let signer_info = SignerInfo::single_direct(Some(sender_public_key), tx_info.sequence_number);
    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(
        (&tx_info.fee_amount).try_into()?,
        tx_info.gas_limit,
    ));

    SignDoc::new(&tx_body, &auth_info, &chain_id, tx_info.account_number)
}

/// UniFFI 0.15.2 doesn't support external types for Kotlin yet
#[derive(Debug, thiserror::Error)]
pub enum ErrorWrapper {
    #[error("Error: {report}")]
    EyreReport { report: eyre::Report },
}

/// creates the transaction signing payload (`SignDoc`)
/// with a single Cosmos SDK message
pub fn get_single_msg_sign_payload(
    tx_info: CosmosSDKTxInfo,
    msg: CosmosSDKMsg,
) -> Result<Vec<u8>, ErrorWrapper> {
    get_single_msg_signdoc(tx_info, msg)
        .and_then(|doc| doc.into_bytes())
        .map_err(|report| ErrorWrapper::EyreReport { report })
}

/// creates the transaction signing payload (`SignDoc`)
/// for MsgSend from the Cosmos SDK bank module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
///
/// FIXME: re-organize wasm-bindgen to its own crate in the same workspace
#[wasm_bindgen]
pub fn get_single_bank_send_signdoc(
    tx_info: CosmosSDKTxInfo,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>, JsValue> {
    get_single_msg_sign_payload(
        tx_info,
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// constructs the Cosmos SDK Tx Info
///
/// FIXME: re-organize wasm-bindgen to its own crate in the same workspace
/// FIXME: investigate if the number of args can be reduced
/// (e.g. if wasm-bindgen can directly tak a struct)
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn new_sdk_tx_info(
    sender_pubkey: Vec<u8>,
    account_number: u64,
    sequence_number: u64,
    gas_limit: u64,
    fee_amount: u64,
    fee_denom: String,
    timeout_height: u32,
    memo_note: Option<String>,
    chain_id: String,
    bech32hrp: String,
    coin_type: u32,
) -> Result<CosmosSDKTxInfo, JsValue> {
    if sender_pubkey.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
        return Err(JsValue::from_str("incorrect public key length"));
    }
    Ok(CosmosSDKTxInfo {
        sender_pubkey: PublicKeyBytesWrapper(sender_pubkey),
        account_number,
        sequence_number,
        gas_limit,
        fee_amount: SingleCoin::Other {
            amount: format!("{}", fee_amount),
            denom: fee_denom,
        },
        timeout_height,
        memo_note,
        network: Network::Other {
            chain_id,
            bech32hrp,
            coin_type,
        },
    })
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod tests {
    use crate::*;
    use cosmrs::crypto::secp256k1::SigningKey;
    use cosmrs::proto::{self};
    use prost::Message;

    #[test]
    fn signdoc_construction_works() {
        let sender_private_key = SigningKey::random();
        let sender_public_key = sender_private_key.public_key();

        let sign_doc_raw = get_single_msg_sign_payload(
            CosmosSDKTxInfo {
                sender_pubkey: PublicKeyBytesWrapper(sender_public_key.to_bytes()),
                account_number: 1,
                sequence_number: 0,
                gas_limit: 100_000,
                timeout_height: 9001,
                fee_amount: SingleCoin::ATOM { amount: 1 },
                memo_note: Some("example memo".to_string()),
                network: Network::CosmosHub,
            },
            CosmosSDKMsg::BankSend {
                recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                amount: SingleCoin::ATOM { amount: 1 },
            },
        )
        .expect("ok sign doc");
        assert!(proto::cosmos::tx::v1beta1::SignDoc::decode(&*sign_doc_raw).is_ok());
    }
}
