use defi_wallet_core_common::{bytes_to_hex, EthNetwork, HDWallet, Network, SecretKey, WalletCoin};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

mod cosmos_sdk;
mod ethereum;
mod nft;
mod utils;

pub use cosmos_sdk::*;
pub use ethereum::*;
pub use nft::*;
pub use utils::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// HD Wallet wrapper for Wasm
#[wasm_bindgen]
pub struct Wallet {
    wallet: HDWallet,
}

/// Signing key wrapper for Wasm
#[derive(Clone)]
#[wasm_bindgen]
pub struct PrivateKey {
    key: Arc<SecretKey>,
}

#[wasm_bindgen]
impl PrivateKey {
    /// generates a random private key
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            key: Arc::new(SecretKey::new()),
        }
    }

    /// constructs private key from bytes
    #[wasm_bindgen]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<PrivateKey, JsValue> {
        Ok(Self {
            key: Arc::new(SecretKey::from_bytes(bytes)?),
        })
    }

    /// constructs private key from hex
    #[wasm_bindgen]
    pub fn from_hex(hex: String) -> Result<PrivateKey, JsValue> {
        Ok(Self {
            key: Arc::new(SecretKey::from_hex(hex)?),
        })
    }

    // eth sign message data
    #[wasm_bindgen]
    pub fn sign_eth(&self, message: Vec<u8>, chain_id: u64) -> Result<Vec<u8>, JsValue> {
        let signature = self.key.sign_eth(message.as_ref(), chain_id)?;
        Ok(signature.to_vec())
    }

    /// gets public key to byte array
    #[wasm_bindgen]
    pub fn get_public_key_bytes(&self) -> Vec<u8> {
        self.key.get_public_key_bytes()
    }

    /// gets public key to a hex string without the 0x prefix
    #[wasm_bindgen]
    pub fn get_public_key_hex(&self) -> String {
        self.key.get_public_key_hex()
    }

    /// converts private key to byte array
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes()
    }

    /// converts private key to a hex string without the 0x prefix
    #[wasm_bindgen]
    pub fn to_hex(&self) -> String {
        self.key.to_hex()
    }

    /// converts private to address with coin type
    #[wasm_bindgen]
    pub fn to_address(&self, coin: CoinType) -> Result<String, JsValue> {
        let address = self.key.to_address(coin.into())?;
        Ok(address)
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
    /// Ethereum
    Ethereum,
    /// Cronos
    Cronos,
    /// Polygon
    Polygon,
    /// BinanceSmartChain
    BSC,
}

impl From<CoinType> for WalletCoin {
    fn from(coin: CoinType) -> Self {
        match coin {
            CoinType::CryptoOrgMainnet => WalletCoin::CosmosSDK {
                network: Network::CryptoOrgMainnet,
            },
            CoinType::CryptoOrgTestnet => WalletCoin::CosmosSDK {
                network: Network::CryptoOrgTestnet,
            },
            CoinType::CronosMainnet => WalletCoin::CosmosSDK {
                network: Network::CronosMainnet,
            },
            CoinType::CosmosHub => WalletCoin::CosmosSDK {
                network: Network::CosmosHub,
            },
            CoinType::Ethereum => WalletCoin::Ethereum {
                network: EthNetwork::Mainnet,
            },
            CoinType::BSC => WalletCoin::Ethereum {
                network: EthNetwork::BSC,
            },
            CoinType::Cronos => WalletCoin::Ethereum {
                network: EthNetwork::Cronos,
            },
            CoinType::Polygon => WalletCoin::Ethereum {
                network: EthNetwork::Polygon,
            },
        }
    }
}

#[wasm_bindgen]
pub enum MnemonicWordCount {
    /// Word 12
    Twelve,
    /// Word 18
    Eighteen,
    /// Word 24
    TwentyFour,
}

impl From<MnemonicWordCount> for defi_wallet_core_common::MnemonicWordCount {
    fn from(word_count: MnemonicWordCount) -> Self {
        match word_count {
            MnemonicWordCount::Twelve => defi_wallet_core_common::MnemonicWordCount::Twelve,
            MnemonicWordCount::Eighteen => defi_wallet_core_common::MnemonicWordCount::Eighteen,
            MnemonicWordCount::TwentyFour => defi_wallet_core_common::MnemonicWordCount::TwentyFour,
        }
    }
}

#[wasm_bindgen]
pub fn bytes2hex(data: Vec<u8>) -> String {
    bytes_to_hex(data)
}

#[wasm_bindgen]
impl Wallet {
    /// generate a random wallet (with an optional password)
    #[wasm_bindgen(constructor)]
    pub fn new(
        password: Option<String>,
        word_count: Option<MnemonicWordCount>,
    ) -> Result<Wallet, JsValue> {
        let wallet = HDWallet::generate_wallet(password, word_count.map(|val| val.into()))?;
        Ok(Self { wallet })
    }

    /// recovers/imports HD wallet from a BIP39 backup phrase (English words) and an optional password
    #[wasm_bindgen]
    pub fn recover_wallet(
        mnemonic_phase: String,
        password: Option<String>,
    ) -> Result<Wallet, JsValue> {
        let wallet = HDWallet::recover_wallet(mnemonic_phase, password)?;
        Ok(Self { wallet })
    }

    /// return the default address for a given coin type
    #[wasm_bindgen]
    pub fn get_default_address(&self, coin: CoinType) -> Result<String, JsValue> {
        Ok(self.wallet.get_default_address(coin.into())?)
    }

    /// return the address for a given coin type and index
    #[wasm_bindgen]
    pub fn get_address(&self, coin: CoinType, index: u32) -> Result<String, JsValue> {
        Ok(self.wallet.get_address(coin.into(), index)?)
    }

    /// obtain a signing key for a given derivation path
    /// derivation_path is bip44 key path
    #[wasm_bindgen]
    pub fn get_key(&self, derivation_path: String) -> Result<PrivateKey, JsValue> {
        let key = self.wallet.get_key(derivation_path)?;
        Ok(PrivateKey { key })
    }

    /// obtain a signing key for a given CoinType and index
    #[wasm_bindgen]
    pub fn get_key_from_index(&self, coin: CoinType, index: u32) -> Result<PrivateKey, JsValue> {
        let key = self.wallet.get_key_from_index(coin.into(), index)?;
        Ok(PrivateKey { key })
    }

    /// Get the mnemonic for the wallet
    #[wasm_bindgen]
    pub fn get_backup_mnemonic_phrase(&self) -> Option<String> {
        self.wallet.get_backup_mnemonic_phrase()
    }
}
