use wasm_bindgen::JsValue;

/// wrapper around API errors
#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("HTTP request error: {0}")]
    RequestError(reqwest::Error),
    #[error("Missing result in the JSON-RPC response")]
    MissingResult,
    #[error("Async Runtime error")]
    AsyncRuntimeError,
    #[error("gRPC transport error: {0}")]
    GRPCTransportError(tonic::transport::Error),
    #[error("gRPC error: {0}")]
    GRPCError(tonic::Status),
    #[error("ErrorReport")]
    ErrorReport,
}

impl From<RestError> for JsValue {
    fn from(e: RestError) -> Self {
        JsValue::from_str(&format!("error: {}", e))
    }
}
