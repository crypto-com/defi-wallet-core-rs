use crate::cosmos_sdk::CosmosSDKTxInfoRaw;
use crate::PrivateKey;
use defi_wallet_core_common::{node, transaction};
use wasm_bindgen::prelude::*;

/// creates the signed transaction
/// for `MsgIssueDenom` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_issue_denom_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    name: String,
    schema: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_issue_denom_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        name,
        schema,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgMintNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_mint_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    recipient: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_mint_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        denom_id,
        name,
        uri,
        data,
        recipient,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgEditNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_edit_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_edit_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        denom_id,
        name,
        uri,
        data,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgTransferNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_transfer_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
    recipient: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_transfer_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        denom_id,
        recipient,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgBurnNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_burn_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_burn_signed_tx(tx_info.into(), private_key.key, id, denom_id)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// Grpc Web Client wrapper for Wasm
#[wasm_bindgen]
pub struct GrpcWebClient(node::nft::Client);

impl GrpcWebClient {
    pub fn new(grpc_web_url: String) -> Self {
        Self(node::nft::Client::new(grpc_web_url))
    }
    pub async fn supply(&mut self, denom_id: String, owner: String) -> Result<JsValue, JsValue> {
        let supply = self.0.supply(denom_id, owner).await?;
        JsValue::from_serde(&supply).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn owner(&mut self, denom_id: String, owner: String) -> Result<JsValue, JsValue> {
        let owner = self.0.owner(denom_id, owner).await?;
        JsValue::from_serde(&owner).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn collection(&mut self, denom_id: String) -> Result<JsValue, JsValue> {
        let collection = self.0.collection(denom_id).await?;
        JsValue::from_serde(&collection).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn denom(&mut self, denom_id: String) -> Result<JsValue, JsValue> {
        let denom = self.0.denom(denom_id).await?;
        JsValue::from_serde(&denom).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn denom_by_name(&mut self, denom_name: String) -> Result<JsValue, JsValue> {
        let denom = self.0.denom_by_name(denom_name).await?;
        JsValue::from_serde(&denom).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn denoms(&mut self) -> Result<JsValue, JsValue> {
        let denoms = self.0.denoms().await?;
        JsValue::from_serde(&denoms).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn nft(&mut self, denom_id: String, token_id: String) -> Result<JsValue, JsValue> {
        let nft = self.0.nft(denom_id, token_id).await?;
        JsValue::from_serde(&nft).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }
}
