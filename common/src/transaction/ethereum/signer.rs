// FIXME: Ethereum signer only has one signing function of EIP-712 for now.
#![cfg(feature = "abi-contract")]

use crate::node::ethereum::eip712::Eip712TypedData;
use crate::transaction::ethereum::EthError;
use crate::wallet::SecretKey;
use ethers::prelude::{LocalWallet, H256};
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
    pub fn sign_typed_data(&self, json_typed_data: &str) -> Result<Vec<u8>, EthError> {
        let encoded_data = Eip712TypedData::new(json_typed_data)?.encode()?;
        Ok(self
            .wallet
            .sign_hash(H256::from_slice(&encoded_data), false)
            .to_vec())
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

    #[test]
    fn test_eip712_typed_data_signing() {
        let wallet = HDWallet::recover_wallet(MNEMONIC.to_string(), None).unwrap();
        let secret_key = wallet.get_key("m/44'/118'/0'/0/0".to_string()).unwrap();
        let signer = EthSigner::new(secret_key);
        let signed_data = signer.sign_typed_data(JSON_TYPED_DATA).unwrap();

        assert_eq!(
            signed_data,
            [
                171, 100, 126, 24, 5, 172, 205, 214, 162, 240, 48, 149, 76, 252, 0, 114, 209, 34,
                150, 208, 251, 83, 211, 194, 160, 7, 59, 155, 87, 60, 240, 245, 51, 80, 62, 207,
                10, 200, 242, 54, 215, 47, 46, 80, 12, 141, 0, 27, 235, 185, 249, 215, 224, 199,
                64, 181, 10, 106, 102, 193, 238, 148, 120, 194, 28
            ]
        );
    }
}
