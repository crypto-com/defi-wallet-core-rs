use std::{str::FromStr, sync::Arc};

use crate::contract::*;
use crate::{construct_simple_eth_transfer_tx, EthAmount, EthError, EthNetwork, SecretKey};
use ethers::prelude::{
    Address, Http, LocalWallet, Middleware, Provider, Signer, SignerMiddleware, TransactionReceipt,
    U256,
};
use ethers::utils::format_units;
#[cfg(not(target_arch = "wasm32"))]
use ethers::utils::hex::ToHex;

/// Information needed for querying balance on different common contract types.
/// The balance in the case of ERC721 returns the number of non-fungible tokens
/// of the same type the account holds (e.g. the number of cryptokitties).
pub enum ContractBalance {
    Erc20 {
        contract_address: String,
    },
    Erc721 {
        contract_address: String,
    },
    Erc1155 {
        contract_address: String,
        token_id: String,
    },
}

/// Information needed for transferring tokens on different common contract types
pub enum ContractTransfer {
    Erc20 {
        contract_address: String,
        to_address: String,
        amount_hex: String,
    },
    Erc721 {
        contract_address: String,
        token_id: String,
        from_address: String,
        to_address: String,
    },
    Erc1155 {
        contract_address: String,
        token_id: String,
        from_address: String,
        to_address: String,
        amount_hex: String,
    },
}

/// given the account address, it returns the amount of native token it owns
pub async fn get_eth_balance(address: &str, web3api_url: &str) -> Result<String, EthError> {
    let to = Address::from_str(address).map_err(|_| EthError::HexConversion)?;
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let result = provider
        .get_balance(to, None)
        .await
        .map_err(|_| EthError::BalanceFail)?;
    format_units(result, "ether").map_err(EthError::ParseError)
}

/// given the account address and contract information, it returns the amount of ERC20/ERC721/ERC1155 token it owns
pub async fn get_contract_balance(
    account_address: &str,
    contract_details: ContractBalance,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let address = Address::from_str(account_address).map_err(|_| EthError::HexConversion)?;
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;

    match &contract_details {
        ContractBalance::Erc20 { contract_address }
        | ContractBalance::Erc721 { contract_address } => {
            let contract_address =
                Address::from_str(contract_address).map_err(|_| EthError::HexConversion)?;
            if matches!(contract_details, ContractBalance::Erc20 { .. }) {
                let contract = Erc20Contract::new(contract_address, Arc::new(client));
                contract
                    .balance_of(address)
                    .call()
                    .await
                    .map_err(|_| EthError::ContractError)
            } else {
                let contract = Erc721Contract::new(contract_address, Arc::new(client));
                contract
                    .balance_of(address)
                    .call()
                    .await
                    .map_err(|_| EthError::ContractError)
            }
        }
        ContractBalance::Erc1155 {
            contract_address,
            token_id,
        } => {
            let contract_address =
                Address::from_str(contract_address).map_err(|_| EthError::HexConversion)?;
            let token_id = U256::from_str(token_id).map_err(|_| EthError::HexConversion)?;
            let contract = Erc1155Contract::new(contract_address, Arc::new(client));
            contract
                .balance_of(address, token_id)
                .call()
                .await
                .map_err(|_| EthError::ContractError)
        }
    }
}

/// given the contract transfer details, it'll construct, sign and broadcast
/// a corresponding transfer transaction.
/// If successful, itt returns the transaction receipt.
pub async fn broadcast_contract_transfer_tx(
    transfer_details: ContractTransfer,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let wallet =
        LocalWallet::from(secret_key.get_eth_signing_key()).with_chain_id(network.get_chain_id());
    let client = SignerMiddleware::new(provider, wallet);
    match transfer_details {
        ContractTransfer::Erc20 {
            contract_address,
            to_address,
            amount_hex,
        } => {
            let contract_address =
                Address::from_str(&contract_address).map_err(|_| EthError::HexConversion)?;
            let to_address = Address::from_str(&to_address).map_err(|_| EthError::HexConversion)?;
            let amount = U256::from_str(&amount_hex).map_err(|_| EthError::HexConversion)?;
            let contract = Erc20Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .transfer(to_address, amount)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractTransfer::Erc721 {
            contract_address,
            token_id,
            from_address,
            to_address,
        } => {
            let contract_address =
                Address::from_str(&contract_address).map_err(|_| EthError::HexConversion)?;
            let token_id = U256::from_str(&token_id).map_err(|_| EthError::HexConversion)?;
            let to_address = Address::from_str(&to_address).map_err(|_| EthError::HexConversion)?;
            let from_address =
                Address::from_str(&from_address).map_err(|_| EthError::HexConversion)?;
            let contract = Erc721Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .safe_transfer_from(from_address, to_address, token_id)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractTransfer::Erc1155 {
            contract_address,
            token_id,
            from_address,
            to_address,
            amount_hex,
        } => {
            let contract_address =
                Address::from_str(&contract_address).map_err(|_| EthError::HexConversion)?;
            let token_id = U256::from_str(&token_id).map_err(|_| EthError::HexConversion)?;
            let amount = U256::from_str(&amount_hex).map_err(|_| EthError::HexConversion)?;

            let to_address = Address::from_str(&to_address).map_err(|_| EthError::HexConversion)?;
            let from_address =
                Address::from_str(&from_address).map_err(|_| EthError::HexConversion)?;
            let contract = Erc1155Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .safe_transfer_from(
                    from_address,
                    to_address,
                    token_id,
                    amount,
                    Default::default(),
                )
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
    }
}

/// given the plain transfer details, it'll construct, sign and broadcast
/// a corresponding transaction.
/// If successful, itt returns the transaction receipt.
pub async fn broadcast_sign_eth_tx(
    to_hex: &str,
    amount: EthAmount,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let tx = construct_simple_eth_transfer_tx(to_hex, amount)?;
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let wallet =
        LocalWallet::from(secret_key.get_eth_signing_key()).with_chain_id(network.get_chain_id());
    let client = SignerMiddleware::new(provider, wallet);

    let pending_tx = client
        .send_transaction(tx, None)
        .await
        .map_err(|_e| EthError::SendTxFail)?;
    let tx_receipt = pending_tx
        .await
        .map_err(|_| EthError::SendTxFail)?
        .ok_or(EthError::MempoolDrop)?;
    Ok(tx_receipt)
}

/// Returns the corresponding account's native token balance
/// formatted in _ETH decimals_ (e.g. "1.50000...") wrapped as string
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn get_eth_balance_blocking(address: &str, web3api_url: &str) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_eth_balance(address, web3api_url))
}

/// Returns the corresponding account's contract token balance in a hexadecimal string,
/// i.e. in its base units unformatted
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn get_contract_balance_blocking(
    account_address: &str,
    contract_details: ContractBalance,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(get_contract_balance(
        account_address,
        contract_details,
        web3api_url,
    ))?;
    Ok(result.to_string())
}

/// given the plain transfer details, it'll construct, sign and broadcast
/// a corresponding transaction.
/// If successful, itt returns the transaction hash/id.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_sign_eth_tx_blocking(
    to_hex: &str,
    amount: EthAmount,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_sign_eth_tx(
        to_hex,
        amount,
        network,
        secret_key,
        web3api_url,
    ))?;
    Ok(result.transaction_hash.encode_hex())
}

/// given the contract transfer details, it'll construct, sign and broadcast
/// a corresponding transfer transaction.
/// If successful, itt returns the transaction hash/id.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_contract_transfer_tx_blocking(
    transfer_details: ContractTransfer,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_contract_transfer_tx(
        transfer_details,
        network,
        secret_key,
        web3api_url,
    ))?;
    Ok(result.transaction_hash.encode_hex())
}
