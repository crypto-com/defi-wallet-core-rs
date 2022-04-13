// FIXME: Ethereum signer only has one signing function of EIP-712 for now.
#![cfg(feature = "abi-contract")]

use crate::node::ethereum::eip712::Eip712TypedData;
use crate::transaction::ethereum::EthError;
use crate::wallet::SecretKey;
use ethers::prelude::{LocalWallet, H256};
use ethers::utils::hash_message;
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

    /// Sign an arbitrary message as per EIP-191.
    pub fn personal_sign(&self, message: &str) -> String {
        let hash = hash_message(message);
        self.wallet.sign_hash(hash, false).to_string()
    }

    /// Sign an EIP-712 typed data from a JSON string of specified schema as below. The field
    /// `domain`, `message`, `primaryType` and `types` are all mandatory as described in
    /// [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
    ///
    /// {
    ///   "domain": {
    ///     "name": "Ether Mail",
    ///     "version": "1",
    ///     "chainId": 1,
    ///     "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
    ///   },
    ///   "message": {
    ///     "name": "Bob",
    ///     "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
    ///   }
    ///   "primaryType": "Person",
    ///   "types": {
    ///     "Person": [
    ///       {
    ///         "name": "name",
    ///         "type": "string"
    ///       },
    ///       {
    ///         "name": "wallet",
    ///         "type": "address"
    ///       }
    ///     ]
    ///   }
    /// }
    pub fn sign_typed_data(&self, json_typed_data: &str) -> Result<String, EthError> {
        let encoded_data = Eip712TypedData::new(json_typed_data)?.encode()?;
        Ok(self
            .wallet
            .sign_hash(H256::from_slice(&encoded_data), false)
            .to_string())
    }
}

#[cfg(test)]
mod ethereum_signing_tests {
    use super::*;
    use crate::wallet::HDWallet;

    const MNEMONIC: &str = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief";

    const JSON_TYPED_DATA: &str = r#"
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

    fn get_signer() -> EthSigner {
        let wallet = HDWallet::recover_wallet(MNEMONIC.to_string(), None).unwrap();
        let secret_key = wallet.get_key("m/44'/118'/0'/0/0".to_string()).unwrap();
        EthSigner::new(secret_key)
    }

    #[test]
    fn test_eip191_personal_signing() {
        let signature = get_signer().personal_sign("0xdeadbeaf");
        assert_eq!(signature, "0d9851845464c72fc39829788368cb83e35b659511080ae189f763c9f6fb0e7411020b0f49aee68f2e57158ea1fe4d69b98b381b41480a92fe8095c0ebab6aaf1c");
    }

    #[test]
    fn test_eip712_typed_data_signing() {
        let signature = get_signer().sign_typed_data(JSON_TYPED_DATA).unwrap();
        assert_eq!(signature, "ab647e1805accdd6a2f030954cfc0072d12296d0fb53d3c2a0073b9b573cf0f533503ecf0ac8f236d72f2e500c8d001bebb9f9d7e0c740b50a6a66c1ee9478c21c");
    }
}
