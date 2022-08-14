use crate::{format_to_js_error, PrivateKey};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use defi_wallet_core_common::{
    broadcast_tx_sync, build_signed_msg_tx, get_account_balance, get_account_details, node,
    BalanceApiVersion, CosmosSDKMsg, CosmosSDKTxInfo, Network, SingleCoin, TimeoutHeight,
};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

mod signer;

pub use signer::*;

/// Cosmos client
#[wasm_bindgen]
pub struct CosmosClient {
    config: CosmosClientConfig,
}

#[wasm_bindgen]
impl CosmosClient {
    /// Create an instance.
    #[wasm_bindgen(constructor)]
    pub fn new(config: CosmosClientConfig) -> Self {
        Self { config }
    }

    /// Retrieve the account balance for a given address and a denom.
    /// api-version: https://github.com/cosmos/cosmos-sdk/releases/tag/v0.42.11
    /// - 0 means before 0.42.11 or 0.44.4
    /// - >=1 means after 0.42.11 or 0.44.4
    /// TODO: switch to grpc-web
    pub fn query_account_balance(
        &self,
        address: String,
        denom: String,
        api_version: u8,
    ) -> Promise {
        let api_url = self.config.api_url.to_owned();
        future_to_promise(async move {
            let api_version = if api_version == 0 {
                BalanceApiVersion::Old
            } else {
                BalanceApiVersion::New
            };
            let account_details =
                get_account_balance(&api_url, &address, &denom, api_version).await?;
            JsValue::from_serde(&account_details).map_err(format_to_js_error)
        })
    }

    /// Retrieve the account details (e.g. sequence and account number) for a given address.
    /// TODO: switch to grpc-web
    pub fn query_account_details(&self, address: String) -> Promise {
        let api_url = self.config.api_url.to_owned();
        future_to_promise(async move {
            let account_details = get_account_details(&api_url, &address).await?;
            JsValue::from_serde(&account_details).map_err(format_to_js_error)
        })
    }

    /// Broadcast a signed transaction.
    #[wasm_bindgen]
    pub fn broadcast_tx(&self, raw_signed_tx: Vec<u8>) -> Promise {
        let tendermint_rpc_url = self.config.tendermint_rpc_url.to_owned();
        future_to_promise(async move {
            let resp = broadcast_tx_sync(&tendermint_rpc_url, raw_signed_tx)
                .await?
                .into_result()
                .map_err(format_to_js_error)?;

            if let tendermint::abci::Code::Err(_) = resp.code {
                return Err(JsValue::from_serde(&resp).map_err(format_to_js_error)?);
            }

            JsValue::from_serde(&resp).map_err(format_to_js_error)
        })
    }
}

/// Cosmos client configuration
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct CosmosClientConfig {
    api_url: String,
    tendermint_rpc_url: String,
}

#[wasm_bindgen]
impl CosmosClientConfig {
    /// Create an instance.
    #[wasm_bindgen(constructor)]
    pub fn new(api_url: String, tendermint_rpc_url: String) -> Self {
        Self {
            api_url,
            tendermint_rpc_url,
        }
    }
}

/// Cosmos message wrapper
#[wasm_bindgen]
pub struct CosmosMsg {
    msg: CosmosSDKMsg,
}

#[wasm_bindgen]
impl CosmosMsg {
    /// construct BankSend message
    #[wasm_bindgen]
    pub fn build_bank_send_msg(recipient_address: String, amount: u64, denom: String) -> Self {
        Self {
            msg: CosmosSDKMsg::BankSend {
                recipient_address,
                amount: SingleCoin::Other {
                    amount: amount.to_string(),
                    denom,
                },
            },
        }
    }

    /// construct NftIssueDenom message
    #[wasm_bindgen]
    pub fn build_nft_issue_denom_msg(id: String, name: String, schema: String) -> Self {
        Self {
            msg: CosmosSDKMsg::NftIssueDenom { id, name, schema },
        }
    }

    /// construct NftMint message
    #[wasm_bindgen]
    pub fn build_nft_mint_msg(
        id: String,
        denom_id: String,
        name: String,
        uri: String,
        data: String,
        recipient: String,
    ) -> Self {
        Self {
            msg: CosmosSDKMsg::NftMint {
                id,
                denom_id,
                name,
                uri,
                data,
                recipient,
            },
        }
    }

    /// construct NftEdit message
    #[wasm_bindgen]
    pub fn build_nft_edit_msg(
        id: String,
        denom_id: String,
        name: String,
        uri: String,
        data: String,
    ) -> Self {
        Self {
            msg: CosmosSDKMsg::NftEdit {
                id,
                denom_id,
                name,
                uri,
                data,
            },
        }
    }

    /// construct NftTransfer message
    pub fn build_nft_transfer_msg(id: String, denom_id: String, recipient: String) -> Self {
        Self {
            msg: CosmosSDKMsg::NftTransfer {
                id,
                denom_id,
                recipient,
            },
        }
    }

    /// construct NftBurn message
    pub fn build_nft_burn_msg(id: String, denom_id: String) -> Self {
        Self {
            msg: CosmosSDKMsg::NftBurn { id, denom_id },
        }
    }

    /// construct StakingBeginRedelegate message
    pub fn build_staking_begin_redelegate_msg(
        validator_src_address: String,
        validator_dst_address: String,
        amount: u64,
        denom: String,
    ) -> Self {
        Self {
            msg: CosmosSDKMsg::StakingBeginRedelegate {
                validator_src_address,
                validator_dst_address,
                amount: SingleCoin::Other {
                    amount: amount.to_string(),
                    denom,
                },
            },
        }
    }

    /// construct StakingDelegate message
    pub fn build_staking_delegate_msg(
        validator_address: String,
        amount: u64,
        denom: String,
    ) -> Self {
        Self {
            msg: CosmosSDKMsg::StakingDelegate {
                validator_address,
                amount: SingleCoin::Other {
                    amount: amount.to_string(),
                    denom,
                },
            },
        }
    }

    /// construct StakingUndelegate message
    pub fn build_staking_undelegate_msg(
        validator_address: String,
        amount: u64,
        denom: String,
    ) -> Self {
        Self {
            msg: CosmosSDKMsg::StakingUndelegate {
                validator_address,
                amount: SingleCoin::Other {
                    amount: amount.to_string(),
                    denom,
                },
            },
        }
    }

    /// construct DistributionSetWithdrawAddress message
    pub fn build_distribution_set_withdraw_address_msg(withdraw_address: String) -> Self {
        Self {
            msg: CosmosSDKMsg::DistributionSetWithdrawAddress { withdraw_address },
        }
    }

    /// construct DistributionWithdrawDelegatorReward message
    pub fn build_distribution_withdraw_delegator_reward_msg(validator_address: String) -> Self {
        Self {
            msg: CosmosSDKMsg::DistributionWithdrawDelegatorReward { validator_address },
        }
    }

    /// construct IbcTransfer message
    #[allow(clippy::too_many_arguments)]
    pub fn build_ibc_transfer_msg(
        receiver: String,
        source_port: String,
        source_channel: String,
        denom: String,
        token: u64,
        revision_height: u64,
        revision_number: u64,
        timeout_timestamp: u64,
    ) -> Self {
        Self {
            msg: CosmosSDKMsg::IbcTransfer {
                receiver,
                source_port,
                source_channel,
                token: SingleCoin::Other {
                    amount: token.to_string(),
                    denom,
                },
                timeout_height: TimeoutHeight {
                    revision_height,
                    revision_number,
                },
                timeout_timestamp,
            },
        }
    }
}

/// Cosmos transaction
#[wasm_bindgen]
pub struct CosmosTx {
    msgs: Vec<CosmosMsg>,
}

#[wasm_bindgen]
impl CosmosTx {
    /// Create a Cosmos transaction
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { msgs: vec![] }
    }

    /// Add a Cosmos message to transaction
    #[wasm_bindgen]
    pub fn add_msg(&mut self, msg: CosmosMsg) {
        self.msgs.push(msg);
    }

    /// Get the count of pending messages
    #[wasm_bindgen]
    pub fn get_msg_count(&self) -> usize {
        self.msgs.len()
    }

    /// Sign the transaction and move out all pending messages
    #[wasm_bindgen]
    pub fn sign_into(
        &mut self,
        private_key: PrivateKey,
        tx_info: CosmosSDKTxInfoRaw,
    ) -> Result<Vec<u8>, JsValue> {
        Ok(build_signed_msg_tx(
            tx_info.into(),
            self.msgs.drain(..).map(|m| m.msg).collect(),
            private_key.key,
        )?)
    }
}

/// the common transaction data needed for Cosmos SDK transactions
/// (raw duplicate needed for Wasm -- TODO: unify common structures?)
#[wasm_bindgen(getter_with_clone)]
pub struct CosmosSDKTxInfoRaw {
    /// global account number of the sender
    pub account_number: u64,
    /// equivalent of "account nonce"
    pub sequence_number: u64,
    /// the maximum gas limit
    pub gas_limit: u64,
    /// the amount fee to be paid (gas_limit * gas_price)
    pub fee_amount: u64,
    /// the fee's denomination
    pub fee_denom: String,
    /// transaction timeout
    pub timeout_height: u32,
    /// optional memo
    pub memo_note: Option<String>,
    /// the network chain id
    pub chain_id: String,
    /// bech32 human readable prefix
    pub bech32hrp: String,
    /// the coin type to use
    pub coin_type: u32,
}

#[wasm_bindgen]
impl CosmosSDKTxInfoRaw {
    /// constructor for JS -- TODO: some builder API wrapper?
    #[wasm_bindgen(constructor)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_number: u64,
        sequence_number: u64,
        gas_limit: u64,
        fee_amount: u64,
        fee_denom: String,
        timeout_height: u32,
        memo_note: Option<String>,
        chain_id: String,
        bech32hrp: String,
        coin_type: u32,
    ) -> Self {
        Self {
            account_number,
            sequence_number,
            gas_limit,
            fee_amount,
            fee_denom,
            timeout_height,
            memo_note,
            chain_id,
            bech32hrp,
            coin_type,
        }
    }
}

impl From<CosmosSDKTxInfoRaw> for CosmosSDKTxInfo {
    fn from(info: CosmosSDKTxInfoRaw) -> Self {
        CosmosSDKTxInfo {
            account_number: info.account_number,
            sequence_number: info.sequence_number,
            gas_limit: info.gas_limit,
            fee_amount: SingleCoin::Other {
                amount: info.fee_amount.to_string(),
                denom: info.fee_denom,
            },
            timeout_height: info.timeout_height,
            memo_note: info.memo_note,
            network: Network::Other {
                chain_id: info.chain_id,
                coin_type: info.coin_type,
                bech32hrp: info.bech32hrp,
            },
        }
    }
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
        JsValue::from_serde(&supply).map_err(format_to_js_error)
    }

    pub async fn owner(
        &mut self,
        denom_id: String,
        owner: String,
        pagination: Option<PageRequest>,
    ) -> Result<JsValue, JsValue> {
        let owner = self.0.owner(denom_id, owner, pagination).await?;
        JsValue::from_serde(&owner).map_err(format_to_js_error)
    }

    pub async fn collection(
        &mut self,
        denom_id: String,
        pagination: Option<PageRequest>,
    ) -> Result<JsValue, JsValue> {
        let collection = self.0.collection(denom_id, pagination).await?;
        JsValue::from_serde(&collection).map_err(format_to_js_error)
    }

    pub async fn denom(&mut self, denom_id: String) -> Result<JsValue, JsValue> {
        let denom = self.0.denom(denom_id).await?;
        JsValue::from_serde(&denom).map_err(format_to_js_error)
    }

    pub async fn denom_by_name(&mut self, denom_name: String) -> Result<JsValue, JsValue> {
        let denom = self.0.denom_by_name(denom_name).await?;
        JsValue::from_serde(&denom).map_err(format_to_js_error)
    }

    pub async fn denoms(&mut self, pagination: Option<PageRequest>) -> Result<JsValue, JsValue> {
        let denoms = self.0.denoms(pagination).await?;
        JsValue::from_serde(&denoms).map_err(format_to_js_error)
    }

    pub async fn nft(&mut self, denom_id: String, token_id: String) -> Result<JsValue, JsValue> {
        let nft = self.0.nft(denom_id, token_id).await?;
        JsValue::from_serde(&nft).map_err(format_to_js_error)
    }
}
