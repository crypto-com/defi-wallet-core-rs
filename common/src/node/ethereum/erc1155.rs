use super::address_from_str;
use crate::contract::{Contract, ContractCall};
use crate::provider::get_ethers_provider;
use crate::{u256_from_str, EthError};
use ethers::prelude::{Address, U256};
pub async fn get_uri(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
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

pub async fn get_is_approved_for_all(
    contract_address: &str,
    owner: &str,
    operator: &str,
    web3api_url: &str,
) -> Result<bool, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc1155(contract_address, client)?;
    let owner = address_from_str(owner)?;
    let operator = address_from_str(operator)?;
    let call = contract.is_approved_for_all(owner, operator);
    ContractCall::from(call).call().await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_is_approved_for_all_blocking(
    contract_address: &str,
    owner: &str,
    operator: &str,
    web3api_url: &str,
) -> Result<bool, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_is_approved_for_all(
        contract_address,
        owner,
        operator,
        web3api_url,
    ))
}

pub async fn get_balance_of_batch(
    contract_address: &str,
    account_addresses: Vec<&str>,
    token_ids: Vec<&str>,
    web3api_url: &str,
) -> Result<Vec<U256>, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc1155(contract_address, client)?;
    let account_addresses = account_addresses
        .iter()
        .map(|val| address_from_str(val))
        .collect::<Result<Vec<Address>, _>>()?;
    let token_ids = token_ids
        .iter()
        .map(|val| u256_from_str(val))
        .collect::<Result<Vec<U256>, _>>()?;
    let call = contract.balance_of_batch(account_addresses, token_ids);
    ContractCall::from(call).call().await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_balance_of_batch_blocking(
    contract_address: &str,
    account_addresses: Vec<&str>,
    token_ids: Vec<&str>,
    web3api_url: &str,
) -> Result<Vec<U256>, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_balance_of_batch(
        contract_address,
        account_addresses,
        token_ids,
        web3api_url,
    ))
}
