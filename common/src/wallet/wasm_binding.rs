#![cfg(target_arch = "wasm32")]

use crate::{HdWrapError, SecretKeyWrapError};
use wasm_bindgen::JsValue;

impl From<HdWrapError> for JsValue {
    fn from(error: HdWrapError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}

impl From<SecretKeyWrapError> for JsValue {
    fn from(error: SecretKeyWrapError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}
