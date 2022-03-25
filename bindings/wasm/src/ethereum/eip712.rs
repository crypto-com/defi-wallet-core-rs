use crate::utils::format_to_js_error;
use defi_wallet_core_common::eip712;
use wasm_bindgen::prelude::*;

/// EIP-712 domain
#[wasm_bindgen]
pub struct Eip712Domain {
    internal: eip712::Eip712Domain,
}

#[wasm_bindgen]
impl Eip712Domain {
    /// Contruct an EIP-712 domain.
    #[wasm_bindgen(constructor)]
    pub fn new(
        chain_id: u64,
        name: String,
        version: String,
        verifying_contract: String,
        salt: Option<String>,
    ) -> Result<Eip712Domain, JsValue> {
        let internal = eip712::Eip712Domain::new(chain_id, name, version, verifying_contract, salt)
            .map_err(format_to_js_error)?;
        Ok(Self { internal })
    }
}

/// EIP-712 typed data
#[wasm_bindgen]
pub struct Eip712TypedData {
    internal: eip712::Eip712TypedData,
}

#[wasm_bindgen]
impl Eip712TypedData {
    /// Contruct an EIP-712 typed data.
    #[wasm_bindgen(constructor)]
    pub fn new(domain: Eip712Domain) -> Result<Eip712TypedData, JsValue> {
        let internal = eip712::Eip712TypedData::new(domain.internal);
        Ok(Self { internal })
    }

    /// Encode the typed data.
    #[wasm_bindgen]
    pub fn encode(&self) -> Result<String, JsValue> {
        self.internal.encode().map_err(format_to_js_error)
    }
}
