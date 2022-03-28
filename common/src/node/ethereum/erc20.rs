use super::contract::{Contract, ContractCall};
use crate::EthError;
use ethers::prelude::{Http, Provider};

pub async fn get_name(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.name();
    ContractCall::new_call(call).call().await
}

pub async fn get_symbol(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.symbol();
    ContractCall::new_call(call).call().await
}

pub async fn get_decimals(contract_address: &str, web3api_url: &str) -> Result<u8, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract = Contract::new_erc20(contract_address, client)?;
    let call = contract.decimals();
    ContractCall::new_call(call).call().await
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
