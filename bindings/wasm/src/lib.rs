use std::sync::Arc;

use defi_wallet_core_common::{
    build_signed_single_msg_tx, get_single_msg_sign_payload, CosmosSDKMsg, CosmosSDKTxInfo,
    HDWallet, Network, PublicKeyBytesWrapper, SecretKey, SingleCoin, WalletCoin,
    COMPRESSED_SECP256K1_PUBKEY_SIZE,
};
use wasm_bindgen::prelude::*;
/// wasm utilities
mod utils;

/// HD Wallet wrapper for Wasm
#[wasm_bindgen]
pub struct Wallet {
    wallet: HDWallet,
}

/// Signing key wrapper for Wasm
#[wasm_bindgen]
pub struct PrivateKey {
    key: Arc<SecretKey>,
}

#[wasm_bindgen]
impl PrivateKey {
    /// generate a random signing key
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            key: Arc::new(SecretKey::new()),
        }
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}

/// basic supported coins for wasm
/// TODO: re-work with `Network`
/// (wasm only supports C-style enums)
#[wasm_bindgen]
pub enum CoinType {
    /// Crypto.org Chain mainnet
    CryptoOrgMainnet,
    /// Crypto.org Chain testnet
    CryptoOrgTestnet,
    /// Cronos mainnet beta
    CronosMainnet,
    /// Cosmos Hub mainnet
    CosmosHub,
}

impl From<CoinType> for WalletCoin {
    fn from(coin: CoinType) -> Self {
        WalletCoin::CosmosSDK {
            network: match coin {
                CoinType::CryptoOrgMainnet => Network::CryptoOrgMainnet,
                CoinType::CryptoOrgTestnet => Network::CryptoOrgTestnet,
                CoinType::CronosMainnet => Network::CronosMainnet,
                CoinType::CosmosHub => Network::CosmosHub,
            },
        }
    }
}

#[wasm_bindgen]
impl Wallet {
    /// generate a random wallet (with an optional password)
    #[wasm_bindgen(constructor)]
    pub fn new(password: Option<String>) -> Self {
        Self {
            wallet: HDWallet::generate_wallet(password),
        }
    }

    /// return the default address for a given coin type
    #[wasm_bindgen]
    pub fn get_default_address(&self, coin: CoinType) -> Result<String, JsValue> {
        self.wallet
            .get_default_address(coin.into())
            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    /// obtain a signing key for a given derivation path
    #[wasm_bindgen]
    pub fn get_key(&self, derivation_path: String) -> Result<PrivateKey, JsValue> {
        let key = self
            .wallet
            .get_key(derivation_path)
            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;
        Ok(PrivateKey { key })
    }
}

/// the common transaction data needed for Cosmos SDK transactions
/// (raw duplicate needed for Wasm -- TODO: unify common structures?)
#[wasm_bindgen(getter_with_clone)]
pub struct CosmosSDKTxInfoRaw {
    /// global account number of the sender
    pub account_number: u64,
    /// equivalent of "account nonce"
    pub sequence_number: u64,
    /// the maximum gas limit
    pub gas_limit: u64,
    /// the amount fee to be paid (gas_limit * gas_price)
    pub fee_amount: u64,
    /// the fee's denomination
    pub fee_denom: String,
    /// transaction timeout
    pub timeout_height: u32,
    /// optional memo
    pub memo_note: Option<String>,
    /// the network chain id
    pub chain_id: String,
    /// bech32 human readable prefix
    pub bech32hrp: String,
    /// the coin type to use
    pub coin_type: u32,
}

#[wasm_bindgen]
impl CosmosSDKTxInfoRaw {
    /// constructor for JS -- TODO: some builder API wrapper?
    #[wasm_bindgen(constructor)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
    ) -> Self {
        Self {
            account_number,
            sequence_number,
            gas_limit,
            fee_amount,
            fee_denom,
            timeout_height,
            memo_note,
            chain_id,
            bech32hrp,
            coin_type,
        }
    }
}

impl From<CosmosSDKTxInfoRaw> for CosmosSDKTxInfo {
    fn from(info: CosmosSDKTxInfoRaw) -> Self {
        CosmosSDKTxInfo {
            account_number: info.account_number,
            sequence_number: info.sequence_number,
            gas_limit: info.gas_limit,
            fee_amount: SingleCoin::Other {
                amount: info.fee_amount.to_string(),
                denom: info.fee_denom,
            },
            timeout_height: info.timeout_height,
            memo_note: info.memo_note,
            network: Network::Other {
                chain_id: info.chain_id,
                coin_type: info.coin_type,
                bech32hrp: info.bech32hrp,
            },
        }
    }
}

/// creates the transaction signing payload (`SignDoc`)
/// for MsgSend from the Cosmos SDK bank module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_single_bank_send_signdoc(
    tx_info: CosmosSDKTxInfoRaw,
    sender_pubkey: Vec<u8>,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>, JsValue> {
    if sender_pubkey.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
        return Err(JsValue::from_str("invalid public key length"));
    }
    let pubkey = PublicKeyBytesWrapper(sender_pubkey);
    get_single_msg_sign_payload(
        tx_info.into(),
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
        pubkey,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for MsgSend from the Cosmos SDK bank module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_single_bank_send_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>, JsValue> {
    build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
        private_key.key,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
