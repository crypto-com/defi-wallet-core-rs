use crate::PrivateKey;
use defi_wallet_core_common as common;
use wasm_bindgen::prelude::*;

/// Cosmos Signer
#[wasm_bindgen]
pub struct CosmosSigner {
    inner: common::CosmosSigner,
}

#[wasm_bindgen]
impl CosmosSigner {
    /// Create an instance via a private key.
    #[wasm_bindgen(constructor)]
    pub fn new(private_key: PrivateKey) -> Self {
        Self {
            inner: common::CosmosSigner::new(private_key.key),
        }
    }

    /// Sign the protobuf bytes directly.
    #[wasm_bindgen]
    pub fn sign_direct(
        &self,
        body_bytes: Vec<u8>,
        auth_info_bytes: Vec<u8>,
        chain_id: String,
        account_number: u64,
    ) -> Result<Vec<u8>, JsValue> {
        Ok(self
            .inner
            .sign_direct(body_bytes, auth_info_bytes, chain_id, account_number)?)
    }
}
