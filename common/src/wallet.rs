use std::sync::Arc;

use crate::Network;
use cosmrs::bip32::secp256k1::ecdsa::SigningKey;
use cosmrs::bip32::{DerivationPath, Error, Mnemonic, PrivateKey, Seed, XPrv};
use cosmrs::crypto::PublicKey;
use rand_core::OsRng;
use secrecy::{ExposeSecret, SecretString, Zeroize};
/// describes what coin type to use (for HD derivation or address generation)
pub enum WalletCoin {
    CosmosSDK { network: Network },
}

impl WalletCoin {
    /// get address from a private key
    pub fn derive_address(&self, private_key: &SigningKey) -> Result<String, eyre::Report> {
        let bech32_hrp = match &self {
            WalletCoin::CosmosSDK { network } => network.get_bech32_hrp(),
        };
        let pubkey = PublicKey::from(private_key.public_key());
        pubkey.account_id(bech32_hrp).map(|x| x.to_string())
    }
}

/// BIP32-style wallet that can be backed up to and recovered from BIP39
pub struct HDWallet {
    seed: Seed,
    mnemonic: Option<Mnemonic>,
}

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

    /// returns the default address of the wallet
    pub fn get_default_address(&self, coin: WalletCoin) -> Result<String, HdWrapError> {
        let coin_type = match &coin {
            WalletCoin::CosmosSDK { network } => network.get_coin_type(),
        };
        let bech32_hrp = match &coin {
            WalletCoin::CosmosSDK { network } => network.get_bech32_hrp(),
        };
        let derivation_path: DerivationPath = format!("m/44'/{}'/0'/0/0", coin_type)
            .parse()
            .map_err(HdWrapError::HDError)?;
        let child_xprv =
            XPrv::derive_from_path(&self.seed, &derivation_path).map_err(HdWrapError::HDError)?;
        let pubkey = PublicKey::from(child_xprv.public_key().public_key());
        pubkey
            .account_id(bech32_hrp)
            .map_err(HdWrapError::AccountId)
            .map(|x| x.to_string())
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
        let default_address = wallet
            .get_default_address(WalletCoin::CosmosSDK {
                network: Network::CryptoOrgMainnet,
            })
            .expect("address");
        assert_eq!(
            default_address,
            "cro1u9q8mfpzhyv2s43js7l5qseapx5kt3g2rf7ppf"
        );
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
