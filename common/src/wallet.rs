use std::sync::Arc;

use crate::Network;
use cosmrs::bip32::secp256k1::ecdsa::SigningKey;
use cosmrs::bip32::{DerivationPath, Error, Mnemonic, PrivateKey, Seed, XPrv};
use cosmrs::crypto::PublicKey;
use ethers::utils::hex::ToHex;
use ethers::utils::secret_key_to_address;
use rand_core::OsRng;
use secrecy::{ExposeSecret, SecretString, Zeroize};

/// describes what coin type to use (for HD derivation or address generation)
pub enum WalletCoin {
    CosmosSDK { network: Network },
    Ethereum,
}

impl WalletCoin {
    /// get address from a private key
    pub fn derive_address(&self, private_key: &SigningKey) -> Result<String, eyre::Report> {
        match self {
            WalletCoin::CosmosSDK { network } => {
                let bech32_hrp = network.get_bech32_hrp();
                let pubkey = PublicKey::from(private_key.public_key());
                pubkey.account_id(bech32_hrp).map(|x| x.to_string())
            }
            WalletCoin::Ethereum => {
                // FIXME: remove when `ethers` updates k256
                let private_key_old =
                    ethers::core::k256::ecdsa::SigningKey::from_bytes(&private_key.to_bytes())
                        .expect("two versions of k256 should be byte-compatible");
                let address = secret_key_to_address(&private_key_old);
                let address_hex: String = address.encode_hex();
                Ok(format!("0x{}", address_hex))
            }
        }
    }
}

/// BIP32-style wallet that can be backed up to and recovered from BIP39
pub struct HDWallet {
    seed: Seed,
    mnemonic: Option<Mnemonic>,
}

/// wrapper around HD Wallet errors
#[derive(Debug, thiserror::Error)]
pub enum HdWrapError {
    #[error("The length should be 64-bytes")]
    InvalidLength,
    #[error("HD wallet error: {0}")]
    HDError(Error),
    #[error("AccountId error: {0}")]
    AccountId(eyre::Report),
}

impl HDWallet {
    /// constructs a new HD wallet from the seed value
    /// returns an error if the seed doesn't have a correct length
    pub fn new(mut seed_val: Vec<u8>) -> Result<Self, HdWrapError> {
        const SEED_LEN: usize = 64;
        if seed_val.len() != SEED_LEN {
            Err(HdWrapError::InvalidLength)
        } else {
            let mut seed = [0u8; SEED_LEN];
            seed.copy_from_slice(&seed_val);
            seed_val.zeroize();
            Ok(HDWallet {
                seed: Seed::new(seed),
                mnemonic: None,
            })
        }
    }

    /// generates the HD wallet with a BIP39 backup phrase (English words)
    pub fn generate_wallet(password: Option<String>) -> Self {
        let pass = SecretString::new(password.unwrap_or_default());
        HDWallet::generate_english(pass)
    }

    /// recovers/imports HD wallet from a BIP39 backup phrase (English words)
    pub fn recover_wallet(
        mnemonic_phrase: String,
        password: Option<String>,
    ) -> Result<Self, HdWrapError> {
        let phrase = SecretString::new(mnemonic_phrase);
        let pass = SecretString::new(password.unwrap_or_default());
        Self::recover_english(phrase, pass).map_err(HdWrapError::HDError)
    }

    /// returns the backup mnemonic phrase (if any)
    pub fn get_backup_mnemonic_phrase(&self) -> Option<String> {
        self.mnemonic
            .as_ref()
            .map(|phrase| phrase.phrase().to_owned())
    }

    /// generates the HD wallet and returns the backup phrase
    fn generate_english(password: SecretString) -> Self {
        let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
        let seed = mnemonic.to_seed(password.expose_secret());
        Self {
            seed,
            mnemonic: Some(mnemonic),
        }
    }

    /// recovers the HD wallet from a backup phrase
    fn recover_english(
        mnemonic_phrase: SecretString,
        password: SecretString,
    ) -> Result<Self, Error> {
        let mnemonic = Mnemonic::new(mnemonic_phrase.expose_secret(), Default::default())
            .map_err(|_| Error::Decode)?;
        let seed = mnemonic.to_seed(password.expose_secret());
        Ok(Self {
            seed,
            mnemonic: Some(mnemonic),
        })
    }

    /// returns the address from index in wallet
    pub fn get_address(&self, coin: WalletCoin, index: u32) -> Result<String, HdWrapError> {
        let coin_type = match &coin {
            WalletCoin::CosmosSDK { network } => network.get_coin_type(),
            WalletCoin::Ethereum => 60,
        };
        let derivation_path: DerivationPath = format!("m/44'/{}'/0'/0/{}", coin_type, index)
            .parse()
            .map_err(HdWrapError::HDError)?;

        let child_xprv =
            XPrv::derive_from_path(&self.seed, &derivation_path).map_err(HdWrapError::HDError)?;
        coin.derive_address(child_xprv.private_key())
            .map_err(HdWrapError::AccountId)
    }

    /// returns the default address of the wallet
    pub fn get_default_address(&self, coin: WalletCoin) -> Result<String, HdWrapError> {
        self.get_address(coin, 0)
    }

    /// return the secret key for a given derivation path
    pub fn get_key(&self, derivation_path: String) -> Result<Arc<SecretKey>, HdWrapError> {
        let derivation_path: DerivationPath =
            derivation_path.parse().map_err(HdWrapError::HDError)?;
        let child_xprv =
            XPrv::derive_from_path(&self.seed, &derivation_path).map_err(HdWrapError::HDError)?;
        Ok(Arc::new(SecretKey(child_xprv.private_key().clone())))
    }
}

/// wrapper around secp256k1 signing key
pub struct SecretKey(SigningKey);

impl SecretKey {
    /// generates a random secret key
    pub fn new() -> Self {
        SecretKey(SigningKey::random(&mut OsRng))
    }

    /// get the inner signing key (for CosmRS signing)
    pub fn get_signing_key(&self) -> Box<SigningKey> {
        Box::new(self.0.clone())
    }

    /// get the inner signing key (for ethers signing)
    /// FIXME: remove when `ethers` updates k256
    pub fn get_eth_signing_key(&self) -> ethers::core::k256::ecdsa::SigningKey {
        ethers::core::k256::ecdsa::SigningKey::from_bytes(&self.0.to_bytes())
            .expect("two versions of k256 should be byte-compatible")
    }
}

impl Default for SecretKey {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    use secrecy::SecretString;

    #[test]
    fn hd_wallet_works() {
        let words = "dune car envelope chuckle elbow slight proud fury remove candy uphold puzzle call select sibling sport gadget please want vault glance verb damage gown";
        let phrase = SecretString::from(words.to_string());
        let password = SecretString::from("".to_string());

        let wallet = HDWallet::recover_english(phrase, password).expect("wallet");
        let default_cosmos_address = wallet
            .get_default_address(WalletCoin::CosmosSDK {
                network: Network::CryptoOrgMainnet,
            })
            .expect("address");
        assert_eq!(
            default_cosmos_address,
            "cro1u9q8mfpzhyv2s43js7l5qseapx5kt3g2rf7ppf"
        );
        let default_eth_address = wallet
            .get_default_address(WalletCoin::Ethereum)
            .expect("address");
        assert_eq!(
            default_eth_address,
            "0x2c600e0a72b3ae39e9b27d2e310b180abe779368"
        );
        let cosmos_address = wallet
            .get_address(
                WalletCoin::CosmosSDK {
                    network: Network::CryptoOrgMainnet,
                },
                1,
            )
            .expect("address");
        assert_eq!(cosmos_address, "cro1g8w7w0kdx0hfv4eqhmv8avxnf7qruchg9pk3v2");
        let eth_address = wallet
            .get_address(WalletCoin::Ethereum, 1)
            .expect("address");
        assert_eq!(eth_address, "0x5a64bef6db23fc854e79eea9e630ccb9301629cb");

        let private_key = wallet
            .get_key("m/44'/394'/0'/0/0".to_string())
            .expect("key");
        let raw_key = private_key.0.to_bytes().to_vec();
        let expected_key = [
            212, 154, 121, 125, 182, 59, 97, 193, 72, 209, 118, 126, 97, 111, 241, 92, 61, 217,
            200, 59, 99, 203, 166, 28, 33, 142, 161, 114, 242, 56, 98, 42,
        ]
        .to_vec();
        assert_eq!(raw_key, expected_key);
    }
}
