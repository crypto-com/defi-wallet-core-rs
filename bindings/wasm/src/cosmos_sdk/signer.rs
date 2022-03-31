use crate::PrivateKey;
use defi_wallet_core_common::transaction::cosmos_sdk::signer;
use wasm_bindgen::prelude::*;

/// SignDoc for generating sign bytes from protobuf
#[wasm_bindgen]
pub struct CosmosProtoSignDoc {
    inner: signer::CosmosProtoSignDoc,
}

#[wasm_bindgen]
impl CosmosProtoSignDoc {
    /// Create an instance. User needs to assure protobuf bytes are correct.
    #[wasm_bindgen(constructor)]
    pub fn new(
        body_bytes: Vec<u8>,
        auth_info_bytes: Vec<u8>,
        chain_id: String,
        account_number: u64,
    ) -> Self {
        Self {
            inner: signer::CosmosProtoSignDoc::new(
                body_bytes,
                auth_info_bytes,
                chain_id,
                account_number,
            ),
        }
    }

    /// Sign this SignDoc and produce a Raw transaction.
    #[wasm_bindgen]
    pub fn sign(&self, private_key: &PrivateKey) -> Result<Vec<u8>, JsValue> {
        Ok(self.inner.sign(private_key.key)?.to_bytes())
    }
}
