// FIXME: Ethereum signer only has one signing function of EIP-712 for now.
#![cfg(feature = "abi-contract")]

use crate::node::ethereum::eip712::Eip712TypedData;
use crate::transaction::ethereum::EthError;
use crate::wallet::SecretKey;
use ethers::prelude::{LocalWallet, H256};
use ethers::utils::hash_message;
use std::str::FromStr;
use std::sync::Arc;

/// Ethereum Signer
pub struct EthSigner {
    wallet: LocalWallet,
}

impl EthSigner {
    /// Create an instance via a secret key.
    pub fn new(secret_key: Arc<SecretKey>) -> Self {
        Self {
            wallet: secret_key.get_signing_key().into(),
        }
    }

    /// Sign a hash value directly.
    /// Argument `hash` must be a hex value of 32 bytes (H256).
    /// Return a signature of hex string with prefix `0x`.
    /// The security concern around `eth_sign` is not that the signature could be forged or the key
    /// be stolen, but rather a malicious website could trick a user into signing a message that is
    /// actually a valid transaction, and use it to steal ether or tokens.
    /// `personal_sign` prefixes the message, preventing it from being a valid transaction. Because
    /// of this, it is safer for users.
    pub fn eth_sign_insecure(&self, hash: &str) -> Result<String, EthError> {
        let hash = hash.strip_prefix("0x").unwrap_or(hash);
        let hash = H256::from_str(hash).map_err(|_| EthError::HexConversion)?;
        let signature = self.wallet.sign_hash(hash, false).to_string();
        Ok(format!("0x{signature}"))
    }

    /// Sign an arbitrary message as per EIP-191.
    /// Return a signature of hex string with prefix `0x`.
    pub fn personal_sign(&self, message: &str) -> String {
        let hash = hash_message(message);
        let signature = self.wallet.sign_hash(hash, false).to_string();
        format!("0x{signature}")
    }

    /// Sign an EIP-712 typed data from a JSON string of specified schema as below. The field
    /// `domain`, `message`, `primaryType` and `types` are all mandatory as described in
    /// [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
    /// Return a signature of hex string with prefix `0x`.
    ///
    /// {
    ///   "domain": {
    ///     "name": "Ether Mail",
    ///     "version": "1",
    ///     "chainId": 1,
    ///     "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
    ///   },
    ///   "message": {
    ///     "from": {
    ///       "name": "Cow",
    ///       "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
    ///     },
    ///     "to": {
    ///       "name": "Bob",
    ///       "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
    ///     },
    ///     "contents": "Hello, Bob!"
    ///   },
    ///   "primaryType": "Mail",
    ///   "types": {
    ///     "Mail": [
    ///       { "name": "from", "type": "Person" },
    ///       { "name": "to", "type": "Person" },
    ///       { "name": "contents", "type": "string" }
    ///     ],
    ///     "Person": [
    ///       { "name": "name", "type": "string" },
    ///       { "name": "wallet", "type": "address" }
    ///     ]
    ///   }
    /// }
    pub fn sign_typed_data(&self, json_typed_data: &str) -> Result<String, EthError> {
        let encoded_data = Eip712TypedData::new(json_typed_data)?.encode()?;
        let signature = self
            .wallet
            .sign_hash(H256::from_slice(&encoded_data), false)
            .to_string();
        Ok(format!("0x{signature}"))
    }
}

#[cfg(test)]
mod ethereum_signing_tests {
    use super::*;
    use crate::wallet::HDWallet;

    const MNEMONIC: &str = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief";

    const SIMPLE_JSON_TYPED_DATA: &str = r#"
        {
            "domain": {
                "name": "Ether Person",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "message": {
                "name": "Bob",
                "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
            },
            "primaryType": "Person",
            "types": {
                "EIP712Domain": [
                    { "name": "name", "type": "string" },
                    { "name": "version", "type": "string" },
                    { "name": "chainId", "type": "uint256" },
                    { "name": "verifyingContract", "type": "address" }
                ],
                "Person": [
                    { "name": "name", "type": "string" },
                    { "name": "wallet", "type": "address" }
                ]
            }
        }"#;

    const RECURSIVELY_NESTED_JSON_TYPED_DATA: &str = r#"
        {
            "domain": {
                "name": "Ether Mail",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "message": {
                "from": {
                    "name": "Cow",
                    "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
                },
                "to": {
                    "name": "Bob",
                    "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
                },
                "contents": "Hello, Bob!"
            },
            "primaryType": "Mail",
            "types": {
                "Mail": [
                    { "name": "from", "type": "Person" },
                    { "name": "to", "type": "Person" },
                    { "name": "contents", "type": "string" }
                ],
                "Person": [
                    { "name": "name", "type": "string" },
                    { "name": "wallet", "type": "address" }
                ]
            }
        }"#;

    fn get_signer() -> EthSigner {
        let wallet = HDWallet::recover_wallet(MNEMONIC.to_string(), None).unwrap();
        let secret_key = wallet.get_key("m/44'/118'/0'/0/0".to_string()).unwrap();
        EthSigner::new(secret_key)
    }

    #[test]
    fn test_eth_sign_insecure() {
        let signature = get_signer()
            .eth_sign_insecure("0x01020304050607085152535455565758a1a2a3a4a5a6a7a8f1f2f3f4f5f6f7f8")
            .unwrap();
        assert_eq!(signature, "0x379a17ae4fe51a4a40dab0a8736f9ebd11f0b5465f38192519e7b0e0bdd440137f7c7db0dfa1c78294d6dbf4c0797dcb161ca8f2dea0cd79267833269e1396261c");
    }

    #[test]
    fn test_eip191_personal_sign() {
        let signature = get_signer().personal_sign("Hello World!");
        assert_eq!(signature, "0xb2aba6568054aff557402a3a9369309687019a29bb6180146d7a44043d6f8b19797e9a27c8c2b416a98cab29822927e76602924062725940e4bad56a9971faca1b");
    }

    #[test]
    fn test_eip712_simple_typed_data_sign() {
        let signature = get_signer()
            .sign_typed_data(SIMPLE_JSON_TYPED_DATA)
            .unwrap();
        assert_eq!(signature, "0xab647e1805accdd6a2f030954cfc0072d12296d0fb53d3c2a0073b9b573cf0f533503ecf0ac8f236d72f2e500c8d001bebb9f9d7e0c740b50a6a66c1ee9478c21c");
    }

    #[test]
    fn test_eip712_recursively_nested_typed_data_sign() {
        let signature = get_signer()
            .sign_typed_data(RECURSIVELY_NESTED_JSON_TYPED_DATA)
            .unwrap();
        assert_eq!(signature, "0x7b26b85e4806529d2013146a98282a8bca97712720def119ac5396845b90433c270d07de560dd61c8ad4d53dc94118678ad11b08c9bf52ceac33e2c86d1967601b");
    }
}
