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
