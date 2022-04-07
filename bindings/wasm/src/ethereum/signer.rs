use crate::PrivateKey;
use defi_wallet_core_common as common;
use wasm_bindgen::prelude::*;

/// Ethereum Signer
#[wasm_bindgen]
pub struct EthSigner {
    inner: common::EthSigner,
}

#[wasm_bindgen]
impl EthSigner {
    /// Create an instance via a private key.
    #[wasm_bindgen(constructor)]
    pub fn new(private_key: PrivateKey) -> Self {
        Self {
            inner: common::EthSigner::new(private_key.key),
        }
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
    #[wasm_bindgen(constructor)]
    pub fn sign_typed_data(json_typed_data: &str) -> Result<Vec<u8>, JsValue> {
        Ok(self.inner.sign_typed_data(json_typed_data)?)
    }
}
