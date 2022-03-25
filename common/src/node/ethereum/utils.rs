use std::{str::FromStr, sync::Arc};

use crate::{construct_simple_eth_transfer_tx, EthAmount, EthError, EthNetwork, SecretKey};
use crate::{contract::*, WalletCoin};
use ethers::prelude::{
    Address, Http, LocalWallet, Middleware, Provider, Signer, SignerMiddleware, TransactionReceipt,
    U256,
};
use ethers::utils::format_units;
#[cfg(not(target_arch = "wasm32"))]
use ethers::utils::hex::ToHex;

/// Information needed for approving operator to withdraw from your account on
/// different common contract types.
pub enum ContractApproval {
    Erc20 {
        contract_address: String,
        approved_address: String,
        amount_hex: String,
    },
    Erc721Approve {
        contract_address: String,
        approved_address: String,
        token_id: String,
    },
    Erc721SetApprovalForAll {
        contract_address: String,
        approved_address: String,
        approved: bool,
    },
    Erc1155 {
        contract_address: String,
        approved_address: String,
        approved: bool,
    },
}

/// Information needed for querying balance on different common contract types.
/// The balance in the case of ERC721 returns the number of non-fungible tokens
/// of the same type the account holds (e.g. the number of cryptokitties).
#[derive(Clone)]
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
    Erc20Transfer {
        contract_address: String,
        to_address: String,
        amount_hex: String,
    },
    Erc20TransferFrom {
        contract_address: String,
        from_address: String,
        to_address: String,
        amount_hex: String,
    },
    Erc721TransferFrom {
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
    },
    Erc721SafeTransferFrom {
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
    },
    Erc721SafeTransferFromWithAdditionalData {
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
        additional_data: Vec<u8>,
    },
    Erc1155SafeTransferFrom {
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
        amount_hex: String,
        additional_data: Vec<u8>,
    },
}

/// Information needed for batch transferring tokens on different common contract types
pub enum ContractBatchTransfer {
    Erc1155 {
        contract_address: String,
        from_address: String,
        to_address: String,
        token_ids: Vec<String>,
        hex_amounts: Vec<String>,
        additional_data: Vec<u8>,
    },
}

/// Information needed for querying owner on nft token for Erc721
#[derive(Clone)]
pub enum ContractOwner {
    Erc721 {
        contract_address: String,
        token_id: String,
    },
}

/// given the account address, it returns the amount of native token it owns
pub async fn get_eth_balance(address: &str, web3api_url: &str) -> Result<String, EthError> {
    let to = address_from_str(address)?;
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let result = provider
        .get_balance(to, None)
        .await
        .map_err(|_| EthError::BalanceFail)?;
    format_units(result, "ether").map_err(EthError::ParseError)
}

/// given the account address, it returns the nonce / number of transactions sent from the account
pub async fn get_eth_transaction_count(address: &str, web3api_url: &str) -> Result<U256, EthError> {
    let to = address_from_str(address)?;
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let result = provider
        .get_transaction_count(to, None)
        .await
        .map_err(|_| EthError::BalanceFail)?;
    Ok(result)
}

/// given the account address and contract information, it returns the amount of ERC20/ERC721/ERC1155 token it owns
pub async fn get_contract_balance(
    account_address: &str,
    contract_details: ContractBalance,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let address = address_from_str(account_address)?;
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;

    match &contract_details {
        ContractBalance::Erc20 { contract_address }
        | ContractBalance::Erc721 { contract_address } => {
            let contract_address = address_from_str(contract_address)?;
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
            let contract_address = address_from_str(contract_address)?;
            let token_id = u256_from_str(token_id)?;
            let contract = Erc1155Contract::new(contract_address, Arc::new(client));
            contract
                .balance_of(address, token_id)
                .call()
                .await
                .map_err(|_| EthError::ContractError)
        }
    }
}

/// given the contract information, it returns the owner address
pub async fn get_token_owner(
    contract_owner: ContractOwner,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    match &contract_owner {
        ContractOwner::Erc721 {
            contract_address,
            token_id,
        } => {
            let contract_address = address_from_str(contract_address)?;
            let token_id = u256_from_str(token_id)?;
            let contract = Erc721Contract::new(contract_address, Arc::new(client));
            contract
                .owner_of(token_id)
                .call()
                .await
                .map_err(|_| EthError::ContractError)
        }
    }
}

/// given the contract approval details, it'll construct, sign and broadcast a
/// corresponding approval transaction.
/// If successful, it returns the transaction receipt.
pub async fn broadcast_contract_approval_tx(
    approval_details: ContractApproval,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let (chain_id, _legacy) = network.to_chain_params()?;

    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    match approval_details {
        ContractApproval::Erc20 {
            contract_address,
            approved_address,
            amount_hex,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let approved_address = address_from_str(&approved_address)?;
            let amount = u256_from_str(&amount_hex)?;
            let contract = Erc20Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .approve(approved_address, amount)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractApproval::Erc721Approve {
            contract_address,
            approved_address,
            token_id,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let approved_address = address_from_str(&approved_address)?;
            let token_id = u256_from_str(&token_id)?;
            let contract = Erc721Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .approve(approved_address, token_id)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractApproval::Erc721SetApprovalForAll {
            contract_address,
            approved_address,
            approved,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let approved_address = address_from_str(&approved_address)?;
            let contract = Erc721Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .set_approval_for_all(approved_address, approved)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractApproval::Erc1155 {
            contract_address,
            approved_address,
            approved,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let approved_address = address_from_str(&approved_address)?;
            let contract = Erc1155Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .set_approval_for_all(approved_address, approved)
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

/// given the contract transfer details, it'll construct, sign and broadcast
/// a corresponding transfer transaction.
/// If successful, it returns the transaction receipt.
pub async fn broadcast_contract_transfer_tx(
    transfer_details: ContractTransfer,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let (chain_id, _legacy) = network.to_chain_params()?;

    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    match transfer_details {
        ContractTransfer::Erc20Transfer {
            contract_address,
            to_address,
            amount_hex,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let to_address = address_from_str(&to_address)?;
            let amount = u256_from_str(&amount_hex)?;
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
        ContractTransfer::Erc20TransferFrom {
            contract_address,
            from_address,
            to_address,
            amount_hex,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let from_address = address_from_str(&from_address)?;
            let to_address = address_from_str(&to_address)?;
            let amount = u256_from_str(&amount_hex)?;
            let contract = Erc20Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .transfer_from(from_address, to_address, amount)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractTransfer::Erc721TransferFrom {
            contract_address,
            from_address,
            to_address,
            token_id,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let token_id = u256_from_str(&token_id)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let contract = Erc721Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .transfer_from(from_address, to_address, token_id)
                .send()
                .await
                .map_err(|_| EthError::SendTxFail)?
                .await;
            let tx_receipt = pending_tx
                .map_err(|_| EthError::SendTxFail)?
                .ok_or(EthError::MempoolDrop)?;
            Ok(tx_receipt)
        }
        ContractTransfer::Erc721SafeTransferFrom {
            contract_address,
            from_address,
            to_address,
            token_id,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let token_id = u256_from_str(&token_id)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
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
        ContractTransfer::Erc721SafeTransferFromWithAdditionalData {
            contract_address,
            from_address,
            to_address,
            token_id,
            additional_data,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let token_id = u256_from_str(&token_id)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let contract = Erc721Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .safe_transfer_from_with_data(
                    from_address,
                    to_address,
                    token_id,
                    additional_data.into(),
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
        ContractTransfer::Erc1155SafeTransferFrom {
            contract_address,
            from_address,
            to_address,
            token_id,
            amount_hex,
            additional_data,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let token_id = u256_from_str(&token_id)?;
            let amount = u256_from_str(&amount_hex)?;

            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let contract = Erc1155Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .safe_transfer_from(
                    from_address,
                    to_address,
                    token_id,
                    amount,
                    additional_data.into(),
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

/// given the contract batch-transfer details, it'll construct, sign and
/// broadcast a corresponding transfer transaction.
/// If successful, it returns the transaction receipt.
pub async fn broadcast_contract_batch_transfer_tx(
    details: ContractBatchTransfer,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let (chain_id, _legacy) = network.to_chain_params()?;

    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    match details {
        ContractBatchTransfer::Erc1155 {
            contract_address,
            from_address,
            to_address,
            token_ids,
            hex_amounts,
            additional_data,
        } => {
            let contract_address = address_from_str(&contract_address)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let token_ids = token_ids
                .iter()
                .map(|val| u256_from_str(val))
                .collect::<Result<Vec<U256>, _>>()?;
            let amounts = hex_amounts
                .iter()
                .map(|val| u256_from_str(val))
                .collect::<Result<Vec<U256>, _>>()?;
            let contract = Erc1155Contract::new(contract_address, Arc::new(client));
            let pending_tx = contract
                .safe_batch_transfer_from(
                    from_address,
                    to_address,
                    token_ids,
                    amounts,
                    additional_data.into(),
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
/// If successful, it returns the transaction receipt.
pub async fn broadcast_sign_eth_tx(
    to_hex: &str,
    amount: EthAmount,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;

    let from_address = WalletCoin::Ethereum
        .derive_address(&secret_key.get_signing_key())
        .map_err(|_| EthError::HexConversion)?;
    let tx = construct_simple_eth_transfer_tx(&from_address, to_hex, amount, legacy, chain_id)?;
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let wallet = LocalWallet::from(secret_key.get_signing_key()).with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);

    let pending_tx = client
        .send_transaction(tx, None)
        .await
        .map_err(|_| EthError::SendTxFail)?;
    let tx_receipt = pending_tx
        .await
        .map_err(EthError::BroadcastTxFail)?
        .ok_or(EthError::MempoolDrop)?;
    Ok(tx_receipt)
}

/// broadcast a previously signed ethereum tx async
/// If successful, it returns the transaction receipt
pub async fn broadcast_eth_signed_raw_tx(
    raw_tx: Vec<u8>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let provider = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let pending_tx = provider
        .send_raw_transaction(raw_tx.into())
        .await
        .map_err(EthError::BroadcastTxFail)?;
    let tx_receipt = pending_tx
        .await
        .map_err(EthError::BroadcastTxFail)?
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

/// Returns the corresponding account's nonce / number of transactions
/// sent from it.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn get_eth_transaction_count_blocking(
    address: &str,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_eth_transaction_count(address, web3api_url))
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

/// Returns the owner address of an NFT in a Fixed-size uninterpreted hash type
/// with 20 bytes (160 bits) size.
/// i.e. in its base units unformatted
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn get_token_owner_blocking(
    contract_owner: ContractOwner,
    web3api_url: &str,
) -> Result<Address, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(get_token_owner(contract_owner, web3api_url))?;
    Ok(result)
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

/// given the contract approval details, it'll construct, sign and broadcast a
/// corresponding approval transaction.
/// If successful, it returns the transaction hash/id.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_contract_approval_tx_blocking(
    approval_details: ContractApproval,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_contract_approval_tx(
        approval_details,
        network,
        secret_key,
        web3api_url,
    ))?;
    Ok(result.transaction_hash.encode_hex())
}

/// given the contract transfer details, it'll construct, sign and broadcast
/// a corresponding transfer transaction.
/// If successful, it returns the transaction hash/id.
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

/// given the contract batch-transfer details, it'll construct, sign and
/// broadcast a corresponding transfer transaction.
/// If successful, it returns the transaction hash/id.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_contract_batch_transfer_tx_blocking(
    batch_transfer_details: ContractBatchTransfer,
    network: EthNetwork,
    secret_key: Arc<SecretKey>,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_contract_batch_transfer_tx(
        batch_transfer_details,
        network,
        secret_key,
        web3api_url,
    ))?;
    Ok(result.transaction_hash.encode_hex())
}

/// broadcast a previously signed ethereum tx.
/// If successful, it returns the transaction hash/id.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_eth_signed_raw_tx_blocking(
    raw_tx: Vec<u8>,
    web3api_url: &str,
) -> Result<TransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_eth_signed_raw_tx(raw_tx, web3api_url))?;
    Ok(result)
}

#[inline]
pub fn address_from_str(address_str: &str) -> Result<Address, EthError> {
    Address::from_str(address_str).map_err(|_| EthError::HexConversion)
}

#[inline]
pub fn u256_from_str(u256_str: &str) -> Result<U256, EthError> {
    U256::from_str(u256_str).map_err(|_| EthError::HexConversion)
}
