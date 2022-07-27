use super::address_from_str;
use crate::contract::{Contract, ContractCall};
use crate::provider::get_ethers_provider;
use crate::{u256_from_str, EthError};
use ethers::prelude::{Address, U256};
/// given the contract information, it returns the owner address
pub async fn get_token_owner(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let token_id = u256_from_str(token_id)?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let call = contract.owner_of(token_id);
    ContractCall::from(call).call().await
}

pub async fn get_name(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let call = contract.name();
    ContractCall::from(call).call().await
}

pub async fn get_symbol(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let call = contract.symbol();
    ContractCall::from(call).call().await
}

pub async fn get_token_uri(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let token_id = u256_from_str(token_id)?;
    let call = contract.token_uri(token_id);
    ContractCall::from(call).call().await
}

pub async fn get_approved(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let token_id = u256_from_str(token_id)?;
    let call = contract.get_approved(token_id);
    ContractCall::from(call).call().await
}

pub async fn get_is_approved_for_all(
    contract_address: &str,
    owner: &str,
    operator: &str,
    web3api_url: &str,
) -> Result<bool, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let owner = address_from_str(owner)?;
    let operator = address_from_str(operator)?;
    let call = contract.is_approved_for_all(owner, operator);
    ContractCall::from(call).call().await
}

pub async fn get_total_supply(contract_address: &str, web3api_url: &str) -> Result<U256, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let call = contract.total_supply();
    ContractCall::from(call).call().await
}

pub async fn get_token_by_index(
    contract_address: &str,
    index: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let index = u256_from_str(index)?;
    let call = contract.token_by_index(index);
    ContractCall::from(call).call().await
}

pub async fn get_token_of_owner_by_index(
    contract_address: &str,
    owner: &str,
    index: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc721(contract_address, client)?;
    let owner = address_from_str(owner)?;
    let index = u256_from_str(index)?;
    let call = contract.token_of_owner_by_index(owner, index);
    ContractCall::from(call).call().await
}

/// Returns the owner address of an NFT in a Fixed-size uninterpreted hash type
/// with 20 bytes (160 bits) size.
/// i.e. in its base units unformatted
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn get_token_owner_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(get_token_owner(contract_address, token_id, web3api_url))?;
    Ok(result)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_name_blocking(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_name(contract_address, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_symbol_blocking(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_symbol(contract_address, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_token_uri_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_token_uri(contract_address, token_id, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_approved_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_approved(contract_address, token_id, web3api_url))
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

#[cfg(not(target_arch = "wasm32"))]
pub fn get_total_supply_blocking(
    contract_address: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_total_supply(contract_address, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_token_by_index_blocking(
    contract_address: &str,
    index: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_token_by_index(contract_address, index, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_token_of_owner_by_index_blocking(
    contract_address: &str,
    owner: &str,
    index: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_token_of_owner_by_index(
        contract_address,
        owner,
        index,
        web3api_url,
    ))
}
