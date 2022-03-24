use crate::contract::*;
use crate::{address_from_str, EthError};
use ethers::prelude::{Http, Provider};
use std::sync::Arc;
pub async fn get_name(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract_address = address_from_str(contract_address)?;
    let contract = Erc20Contract::new(contract_address, Arc::new(client));
    contract
        .name()
        .call()
        .await
        .map_err(|_| EthError::ContractError)
}

pub async fn get_symbol(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract_address = address_from_str(contract_address)?;
    let contract = Erc20Contract::new(contract_address, Arc::new(client));
    contract
        .symbol()
        .call()
        .await
        .map_err(|_| EthError::ContractError)
}

pub async fn get_decimals(contract_address: &str, web3api_url: &str) -> Result<u8, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract_address = address_from_str(contract_address)?;
    let contract = Erc20Contract::new(contract_address, Arc::new(client));
    contract
        .decimals()
        .call()
        .await
        .map_err(|_| EthError::ContractError)
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
