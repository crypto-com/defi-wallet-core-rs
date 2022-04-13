use crate::PrivateKey;
use defi_wallet_core_common::EthSigner;
use wasm_bindgen::prelude::*;

/// Sign a hash value directly.
/// Argument `hash` must be a hex value of 32 bytes (H256).
#[wasm_bindgen]
pub fn eth_sign(private_key: PrivateKey, hash: &str) -> Result<String, JsValue> {
    Ok(EthSigner::new(private_key.key).eth_sign(hash)?)
}

/// Sign an arbitrary message as per EIP-191.
#[wasm_bindgen]
pub fn personal_sign(private_key: PrivateKey, message: &str) -> String {
    EthSigner::new(private_key.key).personal_sign(message)
}

/// Sign an EIP-712 typed data from a JSON string of specified schema as below. The field
/// `domain`, `message`, `primaryType` and `types` are all mandatory as described in
/// [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
///
/// {
///   "domain": {
///     "name": "Ether Mail",
///     "version": "1",
///     "chainId": 1,
///     "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
///   },
///   "message": {
///     "name": "Bob",
///     "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
///   }
///   "primaryType": "Person",
///   "types": {
///     "Person": [
///       {
///         "name": "name",
///         "type": "string"
///       },
///       {
///         "name": "wallet",
///         "type": "address"
///       }
///     ]
///   }
/// }
#[wasm_bindgen(js_name = eth_signTypedData)]
pub fn eth_sign_typed_data(
    private_key: PrivateKey,
    json_typed_data: &str,
) -> Result<String, JsValue> {
    Ok(EthSigner::new(private_key.key).sign_typed_data(json_typed_data)?)
}
