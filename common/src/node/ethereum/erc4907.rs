use crate::contract::{Contract, ContractCall};
use crate::provider::get_ethers_provider;
use crate::{u256_from_str, EthError};
use ethers::prelude::{Address, U256};

pub async fn get_user_expires(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc4907(contract_address, client)?;
    let token_id = u256_from_str(token_id)?;
    let call = contract.user_expires(token_id);
    ContractCall::from(call).call().await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_user_expires_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_user_expires(contract_address, token_id, web3api_url))
}

pub async fn get_user_of(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc4907(contract_address, client)?;
    let token_id = u256_from_str(token_id)?;
    let call = contract.user_of(token_id);
    ContractCall::from(call).call().await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_user_of_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_user_of(contract_address, token_id, web3api_url))
}
