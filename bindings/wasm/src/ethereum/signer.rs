use crate::PrivateKey;
use defi_wallet_core_common::EthSigner;
use wasm_bindgen::prelude::*;

/// Sign a hash value directly.
/// Argument `hash` must be a hex value of 32 bytes (H256).
/// Return a signature of hex string without prefix `0x`.
/// The security concern around `eth_sign` is not that the signature could be forged or the key
/// be stolen, but rather a malicious website could trick a user into signing a message that is
/// actually a valid transaction, and use it to steal ether or tokens.
/// `personal_sign` prefixes the message, preventing it from being a valid transaction. Because
/// of this, it is safer for users.
#[wasm_bindgen(js_name = eth_sign)]
pub fn eth_sign_insecure(private_key: PrivateKey, hash: &str) -> Result<String, JsValue> {
    Ok(EthSigner::new(private_key.key).eth_sign_insecure(hash)?)
}

/// Sign an arbitrary message as per EIP-191.
/// Return a signature of hex string without prefix `0x`.
#[wasm_bindgen]
pub fn personal_sign(private_key: PrivateKey, message: &str) -> String {
    EthSigner::new(private_key.key).personal_sign(message)
}

/// Sign an EIP-712 typed data from a JSON string of specified schema as below. The field
/// `domain`, `message`, `primaryType` and `types` are all mandatory as described in
/// [EIP-712](https://eips.ethereum.org/EIPS/eip-712).
/// Return a signature of hex string without prefix `0x`.
///
/// {
///   "domain": {
///     "name": "Ether Mail",
///     "version": "1",
///     "chainId": 1,
///     "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
///   },
///   "message": {
///     "from": {
///       "name": "Cow",
///       "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
///     },
///     "to": {
///       "name": "Bob",
///       "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
///     },
///     "contents": "Hello, Bob!"
///   },
///   "primaryType": "Mail",
///   "types": {
///     "Mail": [
///       { "name": "from", "type": "Person" },
///       { "name": "to", "type": "Person" },
///       { "name": "contents", "type": "string" }
///     ],
///     "Person": [
///       { "name": "name", "type": "string" },
///       { "name": "wallet", "type": "address" }
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
