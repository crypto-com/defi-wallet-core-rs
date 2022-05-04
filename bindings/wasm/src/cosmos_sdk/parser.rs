use defi_wallet_core_common::{new_crypto_org_parser, AuthInfo};
use wasm_bindgen::prelude::*;

/// Cosmos parser
#[wasm_bindgen]
pub struct CosmosParser {
    inner: Box<dyn defi_wallet_core_common::CosmosParser>,
}

#[wasm_bindgen]
impl CosmosParser {
    /// Create a Cosmos parser for `crypto.org`.
    #[wasm_bindgen]
    pub fn new_crypto_org_parser() -> Self {
        Self {
            inner: Box::new(new_crypto_org_parser()),
        }
    }

    #[wasm_bindgen]
    pub fn parse_proto_auto_info(&self, hex_string: &str) -> Result<JsValue, JsValue> {
        Ok(JsValue::from_serde(&self.inner.parse_proto_auto_info().unwrap()).unwrap())
    }
}
