#![cfg(target_arch = "wasm32")]

use crate::{CosmosError, EthError};
use wasm_bindgen::JsValue;

impl From<CosmosError> for JsValue {
    fn from(error: CosmosError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}

impl From<EthError> for JsValue {
    fn from(error: EthError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
