use crate::contract::{Contract, ContractCall};
use crate::{u256_from_str, EthError};
use ethers::prelude::{Http, Provider};

pub async fn get_uri(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract = Contract::new_erc1155(contract_address, client)?;
    let token_id = u256_from_str(token_id)?;
    let call = contract.uri(token_id);
    ContractCall::from(call).call().await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_uri_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_uri(contract_address, token_id, web3api_url))
}
