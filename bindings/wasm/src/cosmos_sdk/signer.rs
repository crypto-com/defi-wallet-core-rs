use crate::PrivateKey;
use defi_wallet_core_common as common;
use wasm_bindgen::prelude::*;

/// SignDoc for generating sign bytes from protobuf
#[wasm_bindgen]
pub struct CosmosProtoSignDoc {
    inner: common::CosmosProtoSignDoc,
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
            inner: common::CosmosProtoSignDoc::new(
                body_bytes,
                auth_info_bytes,
                chain_id,
                account_number,
            ),
        }
    }

    /// Sign this SignDoc and produce a Raw transaction. The protobuf bytes are
    /// moved out after calling this function.
    #[wasm_bindgen]
    pub fn sign_into(self, private_key: &PrivateKey) -> Result<Vec<u8>, JsValue> {
        Ok(self.inner.sign_into(private_key.key.as_ref())?)
    }
}
