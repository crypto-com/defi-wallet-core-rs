use crate::transaction::cosmos_sdk::CosmosError;
use crate::wallet::SecretKey;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::SignDoc;

/// SignDoc for generating sign bytes from protobuf
pub struct CosmosProtoSignDoc {
    inner: SignDoc,
}

impl CosmosProtoSignDoc {
    /// Create an instance. User needs to assure protobuf bytes are correct.
    pub fn new(
        body_bytes: Vec<u8>,
        auth_info_bytes: Vec<u8>,
        chain_id: String,
        account_number: u64,
    ) -> Self {
        Self {
            inner: SignDoc {
                body_bytes,
                auth_info_bytes,
                chain_id,
                account_number,
            },
        }
    }

    /// Sign this SignDoc and produce a Raw transaction. The protobuf bytes are
    /// moved out after calling this function.
    pub fn sign_into(self, secret_key: &SecretKey) -> Result<Vec<u8>, CosmosError> {
        let signing_key = SigningKey::new(Box::new(secret_key.get_signing_key()));
        Ok(self.inner.sign(&signing_key)?.to_bytes()?)
    }
}

#[cfg(test)]
mod cosmos_signer_tests {
    use super::*;
    use crate::wallet::HDWallet;

    const MNEMONIC: &str = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief";

    #[test]
    fn test_protobuf_signing() {
        let wallet = HDWallet::recover_wallet(MNEMONIC.to_string(), None).unwrap();
        let secret_key = wallet.get_key("m/44'/118'/0'/0/0".to_string()).unwrap();

        let body_bytes = vec![
            10, 156, 1, 10, 37, 47, 99, 111, 115, 109, 111, 115, 46, 115, 116, 97, 107, 105, 110,
            103, 46, 118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 85, 110, 100, 101, 108, 101,
            103, 97, 116, 101, 18, 115, 10, 45, 99, 111, 115, 109, 111, 115, 49, 108, 53, 115, 55,
            116, 110, 106, 50, 56, 97, 55, 122, 120, 101, 101, 99, 107, 104, 103, 119, 108, 104,
            106, 121, 115, 56, 100, 108, 114, 114, 101, 102, 103, 113, 114, 52, 112, 106, 18, 52,
            99, 111, 115, 109, 111, 115, 118, 97, 108, 111, 112, 101, 114, 49, 57, 100, 121, 108,
            48, 117, 121, 122, 101, 115, 52, 107, 50, 51, 108, 115, 99, 108, 97, 48, 50, 110, 48,
            54, 102, 99, 50, 50, 104, 52, 117, 113, 52, 101, 54, 52, 107, 51, 26, 12, 10, 5, 117,
            97, 116, 111, 109, 18, 3, 49, 48, 48, 24, 169, 70,
        ];
        let auth_info_bytes = vec![
            10, 78, 10, 70, 10, 31, 47, 99, 111, 115, 109, 111, 115, 46, 99, 114, 121, 112, 116,
            111, 46, 115, 101, 99, 112, 50, 53, 54, 107, 49, 46, 80, 117, 98, 75, 101, 121, 18, 35,
            10, 33, 2, 140, 57, 86, 222, 0, 17, 214, 185, 178, 199, 53, 4, 86, 71, 209, 75, 56,
            230, 53, 87, 228, 151, 252, 2, 93, 233, 161, 122, 87, 41, 197, 32, 18, 4, 10, 2, 8, 1,
            18, 22, 10, 16, 10, 5, 117, 97, 116, 111, 109, 18, 7, 49, 48, 48, 48, 48, 48, 48, 16,
            160, 141, 6,
        ];
        let sign_doc =
            CosmosProtoSignDoc::new(body_bytes, auth_info_bytes, "chaintest".to_string(), 1);
        let signed_data = sign_doc.sign_into(secret_key.as_ref()).unwrap();

        assert_eq!(
            signed_data,
            [
                10, 162, 1, 10, 156, 1, 10, 37, 47, 99, 111, 115, 109, 111, 115, 46, 115, 116, 97,
                107, 105, 110, 103, 46, 118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 85, 110,
                100, 101, 108, 101, 103, 97, 116, 101, 18, 115, 10, 45, 99, 111, 115, 109, 111,
                115, 49, 108, 53, 115, 55, 116, 110, 106, 50, 56, 97, 55, 122, 120, 101, 101, 99,
                107, 104, 103, 119, 108, 104, 106, 121, 115, 56, 100, 108, 114, 114, 101, 102, 103,
                113, 114, 52, 112, 106, 18, 52, 99, 111, 115, 109, 111, 115, 118, 97, 108, 111,
                112, 101, 114, 49, 57, 100, 121, 108, 48, 117, 121, 122, 101, 115, 52, 107, 50, 51,
                108, 115, 99, 108, 97, 48, 50, 110, 48, 54, 102, 99, 50, 50, 104, 52, 117, 113, 52,
                101, 54, 52, 107, 51, 26, 12, 10, 5, 117, 97, 116, 111, 109, 18, 3, 49, 48, 48, 24,
                169, 70, 18, 104, 10, 78, 10, 70, 10, 31, 47, 99, 111, 115, 109, 111, 115, 46, 99,
                114, 121, 112, 116, 111, 46, 115, 101, 99, 112, 50, 53, 54, 107, 49, 46, 80, 117,
                98, 75, 101, 121, 18, 35, 10, 33, 2, 140, 57, 86, 222, 0, 17, 214, 185, 178, 199,
                53, 4, 86, 71, 209, 75, 56, 230, 53, 87, 228, 151, 252, 2, 93, 233, 161, 122, 87,
                41, 197, 32, 18, 4, 10, 2, 8, 1, 18, 22, 10, 16, 10, 5, 117, 97, 116, 111, 109, 18,
                7, 49, 48, 48, 48, 48, 48, 48, 16, 160, 141, 6, 26, 64, 4, 55, 215, 204, 114, 252,
                74, 74, 117, 64, 195, 192, 242, 50, 14, 158, 195, 34, 108, 73, 127, 72, 15, 161,
                195, 148, 192, 253, 19, 203, 136, 32, 56, 51, 17, 190, 201, 239, 156, 53, 216, 197,
                213, 65, 106, 16, 151, 190, 132, 13, 180, 165, 6, 164, 54, 165, 123, 90, 57, 206,
                112, 14, 132, 164
            ]
        );
    }
}
