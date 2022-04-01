#![cfg(target_arch = "wasm32")]

use crate::RestError;
use wasm_bindgen::JsValue;

impl From<RestError> for JsValue {
    fn from(e: RestError) -> Self {
        JsValue::from_str(&format!("error: {}", e))
    }
}
