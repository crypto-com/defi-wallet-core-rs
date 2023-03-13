use crate::EthError;
use ethers::providers::{Http, Provider};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use url::Url;

#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::OnceCell;
#[cfg(not(target_arch = "wasm32"))]
static G_AGENTINFO: OnceCell<String> = OnceCell::new();

#[cfg(not(target_arch = "wasm32"))]
pub fn set_ethers_httpagent(agent: &str) -> Result<(), EthError> {
    if G_AGENTINFO.get().is_none() {
        if G_AGENTINFO.set(agent.to_string()).is_err() {
            return Err(EthError::HttpAgentError);
        }
        return Ok(());
    }
    Err(EthError::HttpAgentError)
}

// urlinfo: url string of the node to connect to, "http://mynode:8545"
// agentinfo: agent string for http header
pub async fn get_ethers_provider(urlinfo: &str) -> Result<Provider<Http>, EthError> {
    let url = Url::parse(urlinfo).map_err(EthError::NodeUrl)?;

    #[cfg(target_arch = "wasm32")]
    let client = reqwest::Client::builder()
        .build()
        .map_err(EthError::ClientError)?;

    #[cfg(not(target_arch = "wasm32"))]
    let client = {
        match G_AGENTINFO.get() {
            Some(v) => reqwest::Client::builder()
                .user_agent(v)
                .timeout(Duration::from_millis(60000))
                .build()
                .map_err(EthError::ClientError)?,
            None => reqwest::Client::builder()
                .user_agent(
                    std::env::var("DEFIWALLETCORE_AGENTINFO")
                        .unwrap_or_else(|_| "defiwalletcore".to_string()),
                )
                .timeout(Duration::from_millis(60000))
                .build()
                .map_err(EthError::ClientError)?,
        }
    };

    let httpprovider = Http::new_with_client(url, client);
    let finalprovider = Provider::new(httpprovider);
    Ok(finalprovider)
}
