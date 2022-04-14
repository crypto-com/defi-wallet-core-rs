use crate::PrivateKey;
use defi_wallet_core_common::CosmosSigner;
use wasm_bindgen::prelude::*;

/// Sign the protobuf bytes directly.
/// As an example, arguments should like:
/// {
///     "chainId": "cosmoshub-4",
///     "accountNumber": "1"
///     "authInfoBytes": "0a0a0a00 ...",
///     "bodyBytes": "0a90010a ...",
/// }
#[wasm_bindgen(js_name = cosmos_signDirect)]
pub fn cosmos_sign_direct(
    private_key: PrivateKey,
    chain_id: &str,
    account_number: &str,
    auth_info_bytes: &str,
    body_bytes: &str,
) -> Result<String, JsValue> {
    Ok(CosmosSigner::new(private_key.key).sign_direct(
        chain_id,
        account_number,
        auth_info_bytes,
        body_bytes,
    )?)
}
