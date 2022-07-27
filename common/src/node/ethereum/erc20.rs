use super::address_from_str;
use crate::contract::{Contract, ContractCall};
use crate::provider::get_ethers_provider;
use crate::EthError;
use ethers::prelude::U256;
pub async fn get_name(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.name();
    ContractCall::from(call).call().await
}

pub async fn get_symbol(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.symbol();
    ContractCall::from(call).call().await
}

pub async fn get_decimals(contract_address: &str, web3api_url: &str) -> Result<u8, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.decimals();
    ContractCall::from(call).call().await
}

pub async fn get_allowance(
    contract_address: &str,
    owner: &str,
    spender: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let owner = address_from_str(owner)?;
    let spender = address_from_str(spender)?;
    let call = contract.allowance(owner, spender);
    ContractCall::from(call).call().await
}

pub async fn get_total_supply(contract_address: &str, web3api_url: &str) -> Result<U256, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.total_supply();
    ContractCall::from(call).call().await
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
pub fn get_decimals_blocking(contract_address: &str, web3api_url: &str) -> Result<u8, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_decimals(contract_address, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_allowance_blocking(
    contract_address: &str,
    owner: &str,
    spender: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_allowance(contract_address, owner, spender, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_total_supply_blocking(
    contract_address: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_total_supply(contract_address, web3api_url))
}
