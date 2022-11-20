use crate::RestError;
use cosmos_sdk_proto::cosmos::bank::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryBalanceResponse};
use serde::{Deserialize, Serialize};

/// The raw balance data from the balance API
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RawRpcBalance {
    /// denomination
    pub denom: String,
    /// the decimal number of coins of a given denomination
    pub amount: String,
}

impl From<QueryBalanceResponse> for RawRpcBalance {
    fn from(response: QueryBalanceResponse) -> Self {
        let balance = response.balance.unwrap_or_default();
        Self {
            amount: balance.amount,
            denom: balance.denom,
        }
    }
}

/// return the balance (async for JS/WASM)
#[cfg(target_arch = "wasm32")]
pub async fn get_account_balance(
    grpc_web_url: &str,
    address: &str,
    denom: &str,
) -> Result<RawRpcBalance, RestError> {
    let mut client = QueryClient::new(tonic_web_wasm_client::Client::new(grpc_web_url.to_string()));
    let request = QueryBalanceRequest {
        address: address.to_string(),
        denom: denom.to_string(),
    };
    Ok(client
        .balance(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner()
        .into())
}

/// return the balance (blocking for other platforms;
/// platform-guarded as JS/WASM doesn't support the reqwest blocking)
#[cfg(not(target_arch = "wasm32"))]
pub fn get_account_balance_blocking(
    grpc_url: &str,
    address: &str,
    denom: &str,
) -> Result<RawRpcBalance, RestError> {
    tokio::runtime::Runtime::new()
        .map_err(|_err| RestError::AsyncRuntimeError)?
        .block_on(async move {
            let mut client = QueryClient::connect(grpc_url.to_string())
                .await
                .map_err(RestError::GRPCTransportError)?;
            let request = QueryBalanceRequest {
                address: address.to_string(),
                denom: denom.to_string(),
            };
            Ok(client
                .balance(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner()
                .into())
        })
}
