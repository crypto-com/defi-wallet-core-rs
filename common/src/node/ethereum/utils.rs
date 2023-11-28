use crate::{
    construct_simple_eth_transfer_tx, EthAmount, EthError, EthNetwork, SecretKey, WalletCoin,
    WalletCoinFunc,
};
use ethers::prelude::{Address, LocalWallet, Middleware, Signer, SignerMiddleware, TxHash};
use ethers::providers::{Http, Provider};
use ethers::types::transaction::eip2718::TypedTransaction;
use std::{str::FromStr, sync::Arc, time::Duration};

#[cfg(not(target_arch = "wasm32"))]
use ethers::utils::hex::ToHex;

use crate::contract::{Contract, ContractCall};

use ethers::prelude::TransactionReceipt as EthersTransactionReceipt;

use ethers::types::U256;

use crate::provider::get_ethers_provider;

use serde::{Deserialize, Serialize};

use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// a subset of `ethers::prelude::::TransactionReceipt` for non-wasm
#[cfg(not(target_arch = "wasm32"))]
pub struct TransactionReceipt {
    pub transaction_hash: Vec<u8>,
    pub transaction_index: String,
    pub block_hash: Vec<u8>,
    pub block_number: String,
    pub cumulative_gas_used: String,
    pub gas_used: String,
    pub contract_address: String,
    pub logs: Vec<String>,
    /// Status: either 1 (success) or 0 (failure)
    pub status: String,
    pub root: Vec<u8>,
    pub logs_bloom: Vec<u8>,
    pub transaction_type: String,
    pub effective_gas_price: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<EthersTransactionReceipt> for TransactionReceipt {
    fn from(src: EthersTransactionReceipt) -> Self {
        TransactionReceipt {
            transaction_hash: src.transaction_hash.to_fixed_bytes().to_vec(),
            transaction_index: src.transaction_index.to_string(),
            block_hash: match src.block_hash {
                Some(block_hash) => block_hash.to_fixed_bytes().to_vec(),
                None => vec![],
            },
            block_number: match src.block_number {
                Some(block_number) => block_number.to_string(),
                None => "".into(),
            },
            cumulative_gas_used: src.cumulative_gas_used.to_string(),
            gas_used: match src.gas_used {
                Some(gas_used) => gas_used.to_string(),
                None => "".into(),
            },
            contract_address: match src.contract_address {
                Some(contract_address) => contract_address.encode_hex(),
                None => "".into(),
            },
            status: match src.status {
                Some(v) => v.to_string(),
                None => "".into(),
            },
            root: match src.root {
                Some(v) => v.to_fixed_bytes().to_vec(),
                None => vec![],
            },
            logs_bloom: src.logs_bloom.to_fixed_bytes().to_vec(),
            transaction_type: match src.transaction_type {
                Some(v) => v.to_string(),
                None => "".into(),
            },
            effective_gas_price: match src.effective_gas_price {
                Some(v) => v.to_string(),
                None => "".into(),
            },
            logs: src
                .logs
                .iter()
                .map(|log| serde_json::to_string(&log).unwrap())
                .collect(),
        }
    }
}

/// Information needed for approving operator to withdraw from your account on
/// different common contract types.
#[derive(Serialize, Deserialize)]
pub enum ContractApproval {
    Erc20 {
        contract_address: String,
        approved_address: String,
        amount: String,
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
#[derive(Serialize, Deserialize)]
pub enum ContractTransfer {
    Erc20Transfer {
        contract_address: String,
        to_address: String,
        amount: String,
    },
    Erc20TransferFrom {
        contract_address: String,
        from_address: String,
        to_address: String,
        amount: String,
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
        amount: String,
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
        amounts: Vec<String>,
        additional_data: Vec<u8>,
    },
}

/// given the account address, it returns the amount of native token it owns
pub async fn get_eth_balance(address: &str, web3api_url: &str) -> Result<U256, EthError> {
    let to = address_from_str(address)?;
    let client = get_ethers_provider(web3api_url).await?;
    let result = client
        .get_balance(to, None)
        .await
        .map_err(|_| EthError::BalanceFail)?;
    Ok(result)
}

/// given the account address, it returns the nonce / number of transactions sent from the account
pub async fn get_eth_transaction_count(address: &str, web3api_url: &str) -> Result<U256, EthError> {
    let to = address_from_str(address)?;
    let client = get_ethers_provider(web3api_url).await?;
    let result = client
        .get_transaction_count(to, None)
        .await
        .map_err(|_| EthError::BalanceFail)?;
    Ok(result)
}

// Wrapper of TxHash to implement TryFrom
#[pin_project]
pub struct TxHashWrapper<'a> {
    tx_hash: TxHash,
    pub web3api_url: String,
    state: TxHashState<'a>,
}

/// A simplified `TxHashState`
enum TxHashState<'a> {
    /// Waiting for interval to elapse before calling API again
    PausedGettingReceipt,

    /// Polling the blockchain for the receipt
    GettingReceipt(
        Pin<Box<dyn Future<Output = Result<Option<EthersTransactionReceipt>, EthError>> + 'a>>,
    ),

    /// TODO If the pending tx required only 1 conf, it will return early. Otherwise it will
    /// proceed to the next state which will poll the block number until there have been
    /// enough confirmations
    CheckingReceipt(Option<EthersTransactionReceipt>),

    Completed,
}

impl<'a> TryFrom<Vec<u8>> for TxHashWrapper<'a> {
    type Error = EthError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(EthError::InvalidTxHash);
        }

        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&value);
        Ok(TxHashWrapper {
            tx_hash: TxHash::from(tx_hash),
            web3api_url: "".to_string(),
            state: TxHashState::PausedGettingReceipt,
        })
    }
}

impl<'a> TryFrom<String> for TxHashWrapper<'a> {
    type Error = EthError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = hex::decode(value).map_err(|_| EthError::HexConversion)?;
        if value.len() != 32 {
            return Err(EthError::InvalidTxHash);
        }

        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&value);
        Ok(TxHashWrapper {
            tx_hash: TxHash::from(tx_hash),
            web3api_url: "".to_string(),
            state: TxHashState::PausedGettingReceipt,
        })
    }
}

impl<'a> Future for TxHashWrapper<'a> {
    type Output = Result<EthersTransactionReceipt, EthError>;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        // println!("Polling...");
        match this.state {
            TxHashState::PausedGettingReceipt => {
                // TODO Wait the polling period so that we do not spam the chain when no
                // new block has been mined
                // let _ready = futures_util::ready!(this.interval.poll_next_unpin(ctx));
                let fut = Box::pin(get_eth_transaction_receipt_by_vec(
                    this.tx_hash.0.to_vec(),
                    this.web3api_url.clone(),
                ));
                *this.state = TxHashState::GettingReceipt(fut);
                ctx.waker().wake_by_ref();
            }
            TxHashState::GettingReceipt(fut) => {
                if let Ok(receipt) = futures_util::ready!(fut.as_mut().poll(ctx)) {
                    // TODO use tracing
                    // println!("Checking receipt for pending tx {:?}", *this.tx_hash);
                    *this.state = TxHashState::CheckingReceipt(receipt)
                } else {
                    *this.state = TxHashState::PausedGettingReceipt
                }
                ctx.waker().wake_by_ref();
            }
            TxHashState::CheckingReceipt(receipt) => {
                if receipt.is_none() {
                    *this.state = TxHashState::PausedGettingReceipt;
                    ctx.waker().wake_by_ref();
                    return Poll::Pending;
                }
                // TODO If we requested more than 1 confirmation, we need to compare the receipt's
                // block number and the current block
                let receipt = receipt.take().unwrap();
                *this.state = TxHashState::Completed;
                return Poll::Ready(Ok(receipt));
            }
            TxHashState::Completed => {
                panic!("polled pending transaction future after completion")
            }
        }
        Poll::Pending
    }
}

// TODO Use get interval in checking receipt
#[allow(dead_code)]
async fn get_interval(web3api_url: String) -> Result<Duration, EthError> {
    let client = get_ethers_provider(&web3api_url).await?;
    let interval = client.get_interval();
    Ok(interval)
}

// TODO Use in wasm
#[allow(dead_code)]
async fn get_block_number(web3api_url: String) -> Result<String, EthError> {
    let client = get_ethers_provider(&web3api_url).await?;

    let block_number = client
        .get_block_number()
        .await
        .map_err(EthError::GetBlockNumberError)?;

    Ok(block_number.to_string())
}

async fn get_eth_transaction_receipt_by_vec(
    tx_hash: Vec<u8>,
    web3api_url: String,
) -> Result<Option<EthersTransactionReceipt>, EthError> {
    let client = get_ethers_provider(&web3api_url).await?;
    let tx_hash = TxHashWrapper::try_from(tx_hash)?;

    let receipt = client
        .get_transaction_receipt(tx_hash.tx_hash)
        .await
        .map_err(EthError::GetTransactionReceiptError)?;

    Ok(receipt)
}

// TODO Use in wasm
#[allow(dead_code)]
async fn get_eth_transaction_receipt_by_string(
    tx_hash: String,
    web3api_url: String,
) -> Result<Option<EthersTransactionReceipt>, EthError> {
    let client = get_ethers_provider(&web3api_url).await?;
    let tx_hash = TxHashWrapper::try_from(tx_hash)?;

    let receipt = client
        .get_transaction_receipt(tx_hash.tx_hash)
        .await
        .map_err(EthError::GetTransactionReceiptError)?;

    Ok(receipt)
}

// TODO Use in wasm
#[allow(dead_code)]
async fn wait_for_transaction_receipt_by_vec(
    tx_hash: Vec<u8>,
    web3api_url: String,
) -> Result<EthersTransactionReceipt, EthError> {
    let mut tx_hash_wrapper = TxHashWrapper::try_from(tx_hash)?;
    tx_hash_wrapper.web3api_url = web3api_url;
    tx_hash_wrapper.await // wait the transaction receipt
}

// TODO Use in wasm
#[allow(dead_code)]
async fn wait_for_transaction_receipt_by_string(
    tx_hash: String,
    web3api_url: String,
) -> Result<EthersTransactionReceipt, EthError> {
    let mut tx_hash_wrapper = TxHashWrapper::try_from(tx_hash)?;
    tx_hash_wrapper.web3api_url = web3api_url;
    tx_hash_wrapper.await // wait the transaction receipt
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_block_number_blocking(web3api_url: String) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_block_number(web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_eth_transaction_receipt_by_string_blocking(
    tx_hash: String,
    web3api_url: String,
) -> Result<Option<EthersTransactionReceipt>, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_eth_transaction_receipt_by_string(tx_hash, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_eth_transaction_receipt_by_vec_blocking(
    tx_hash: Vec<u8>,
    web3api_url: String,
) -> Result<Option<EthersTransactionReceipt>, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_eth_transaction_receipt_by_vec(tx_hash, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn wait_for_transaction_receipt_by_vec_blocking(
    tx_hash: Vec<u8>,
    web3api_url: String,
) -> Result<EthersTransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(wait_for_transaction_receipt_by_vec(tx_hash, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn wait_for_transaction_receipt_by_string_blocking(
    tx_hash: String,
    web3api_url: String,
) -> Result<EthersTransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(wait_for_transaction_receipt_by_string(tx_hash, web3api_url))
}

/// given the account address and contract information, it returns the amount of ERC20/ERC721/ERC1155 token it owns
pub async fn get_contract_balance(
    account_address: &str,
    contract_details: ContractBalance,
    web3api_url: &str,
) -> Result<U256, EthError> {
    let address = address_from_str(account_address)?;
    let client = get_ethers_provider(web3api_url).await?;

    let call = match &contract_details {
        ContractBalance::Erc20 { contract_address }
        | ContractBalance::Erc721 { contract_address } => {
            if matches!(contract_details, ContractBalance::Erc20 { .. }) {
                let contract = Contract::new_erc20(contract_address, client)?;
                contract.balance_of(address)
            } else {
                let contract = Contract::new_erc721(contract_address, client)?;
                contract.balance_of(address)
            }
        }
        ContractBalance::Erc1155 {
            contract_address,
            token_id,
        } => {
            let token_id = u256_from_str(token_id)?;
            let contract = Contract::new_erc1155(contract_address, client)?;
            contract.balance_of(address, token_id)
        }
    };
    ContractCall::from(call).call().await
}

fn create_localwallet_client(
    polling_interval_ms: u64,
    key: Arc<SecretKey>,
    chain_id: u64,
    client: Provider<Http>,
) -> Result<SignerMiddleware<Provider<Http>, LocalWallet>, EthError> {
    let provider = client.interval(Duration::from_millis(polling_interval_ms));
    let ethers_key = key
        .get_eth_signing_key()
        .map_err(|_| EthError::SignatureError)?;
    let wallet = LocalWallet::from(ethers_key).with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    Ok(client)
}

async fn broadcast_contract_approval_tx_common(
    approval_details: ContractApproval,
    network: EthNetwork,
    secret_key: Option<Arc<SecretKey>>,
    web3api_url: &str,
    polling_interval_ms: u64,
) -> Result<(Option<EthersTransactionReceipt>, Option<TypedTransaction>), EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;
    match approval_details {
        ContractApproval::Erc20 {
            contract_address,
            approved_address,
            amount,
        } => {
            let approved_address = address_from_str(&approved_address)?;
            let client = get_ethers_provider(web3api_url).await?;
            let amount = u256_from_dec_str(&amount)?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;
                let contract = Contract::new_erc20(&contract_address, client)?;
                let call = contract.approve(approved_address, amount);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc20(&contract_address, client)?;
                let call = contract.approve(approved_address, amount);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractApproval::Erc721Approve {
            contract_address,
            approved_address,
            token_id,
        } => {
            let approved_address = address_from_str(&approved_address)?;
            let token_id = u256_from_str(&token_id)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;
                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.approve(approved_address, token_id);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.approve(approved_address, token_id);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractApproval::Erc721SetApprovalForAll {
            contract_address,
            approved_address,
            approved,
        } => {
            let approved_address = address_from_str(&approved_address)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.set_approval_for_all(approved_address, approved);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.set_approval_for_all(approved_address, approved);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractApproval::Erc1155 {
            contract_address,
            approved_address,
            approved,
        } => {
            let approved_address = address_from_str(&approved_address)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;
                let contract = Contract::new_erc1155(&contract_address, client)?;
                let call = contract.set_approval_for_all(approved_address, approved);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc1155(&contract_address, client)?;
                let call = contract.set_approval_for_all(approved_address, approved);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
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
    polling_interval_ms: u64,
) -> Result<EthersTransactionReceipt, EthError> {
    let (receipt, _) = broadcast_contract_approval_tx_common(
        approval_details,
        network,
        Some(secret_key),
        web3api_url,
        polling_interval_ms,
    )
    .await?;
    receipt.ok_or_else(|| EthError::ContractSendError("No receipt".to_string()))
}

/// given the contract approval details, it'll construct
/// corresponding approval transaction.
/// If successful, it returns typed transaction.
pub async fn construct_contract_approval_tx(
    approval_details: ContractApproval,
    network: EthNetwork,
    web3api_url: &str,
) -> Result<TypedTransaction, EthError> {
    let (_, tx) =
        broadcast_contract_approval_tx_common(approval_details, network, None, web3api_url, 0)
            .await?;
    tx.ok_or_else(|| EthError::ContractSendError("No tx".to_string()))
}

async fn broadcast_contract_transfer_tx_common(
    transfer_details: ContractTransfer,
    network: EthNetwork,
    secret_key: Option<Arc<SecretKey>>,
    web3api_url: &str,
    polling_interval_ms: u64,
) -> Result<(Option<EthersTransactionReceipt>, Option<TypedTransaction>), EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;

    match transfer_details {
        ContractTransfer::Erc20Transfer {
            contract_address,
            to_address,
            amount,
        } => {
            let to_address = address_from_str(&to_address)?;
            let amount = u256_from_dec_str(&amount)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc20(&contract_address, client)?;
                let call = contract.transfer(to_address, amount);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc20(&contract_address, client)?;
                let call = contract.transfer(to_address, amount);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractTransfer::Erc20TransferFrom {
            contract_address,
            from_address,
            to_address,
            amount,
        } => {
            let from_address = address_from_str(&from_address)?;
            let to_address = address_from_str(&to_address)?;
            let amount = u256_from_dec_str(&amount)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;
                let contract = Contract::new_erc20(&contract_address, client)?;
                let call = contract.transfer_from(from_address, to_address, amount);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc20(&contract_address, client)?;
                let call = contract.transfer_from(from_address, to_address, amount);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractTransfer::Erc721TransferFrom {
            contract_address,
            from_address,
            to_address,
            token_id,
        } => {
            let token_id = u256_from_str(&token_id)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let client = get_ethers_provider(web3api_url).await?;

            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.transfer_from(from_address, to_address, token_id);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.transfer_from(from_address, to_address, token_id);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractTransfer::Erc721SafeTransferFrom {
            contract_address,
            from_address,
            to_address,
            token_id,
        } => {
            let token_id = u256_from_str(&token_id)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let client = get_ethers_provider(web3api_url).await?;

            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.safe_transfer_from(from_address, to_address, token_id);
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.safe_transfer_from(from_address, to_address, token_id);
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractTransfer::Erc721SafeTransferFromWithAdditionalData {
            contract_address,
            from_address,
            to_address,
            token_id,
            additional_data,
        } => {
            let token_id = u256_from_str(&token_id)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.safe_transfer_from_with_from_and_to_and_data(
                    from_address,
                    to_address,
                    token_id,
                    additional_data.into(),
                );
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc721(&contract_address, client)?;
                let call = contract.safe_transfer_from_with_from_and_to_and_data(
                    from_address,
                    to_address,
                    token_id,
                    additional_data.into(),
                );
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
        }
        ContractTransfer::Erc1155SafeTransferFrom {
            contract_address,
            from_address,
            to_address,
            token_id,
            amount,
            additional_data,
        } => {
            let token_id = u256_from_str(&token_id)?;
            let amount = u256_from_dec_str(&amount)?;
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let client = get_ethers_provider(web3api_url).await?;
            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc1155(&contract_address, client)?;
                let call = contract.safe_transfer_from(
                    from_address,
                    to_address,
                    token_id,
                    amount,
                    additional_data.into(),
                );
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc1155(&contract_address, client)?;
                let call = contract.safe_transfer_from(
                    from_address,
                    to_address,
                    token_id,
                    amount,
                    additional_data.into(),
                );
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
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
    polling_interval_ms: u64,
) -> Result<EthersTransactionReceipt, EthError> {
    let (receipt, _) = broadcast_contract_transfer_tx_common(
        transfer_details,
        network,
        Some(secret_key),
        web3api_url,
        polling_interval_ms,
    )
    .await?;
    receipt.ok_or_else(|| EthError::ContractSendError("No receipt".to_string()))
}

/// given the contract transfer details, it'll construct
/// a corresponding transfer transaction.
/// If successful, it returns the typed transaction.
pub async fn construct_contract_transfer_tx(
    transfer_details: ContractTransfer,
    network: EthNetwork,
    web3api_url: &str,
) -> Result<TypedTransaction, EthError> {
    let (_, tx) =
        broadcast_contract_transfer_tx_common(transfer_details, network, None, web3api_url, 0)
            .await?;
    tx.ok_or_else(|| EthError::ContractSendError("No tx".to_string()))
}

async fn broadcast_contract_batch_transfer_tx_common(
    details: ContractBatchTransfer,
    network: EthNetwork,
    secret_key: Option<Arc<SecretKey>>,
    web3api_url: &str,
    polling_interval_ms: u64,
) -> Result<(Option<EthersTransactionReceipt>, Option<TypedTransaction>), EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;
    match details {
        ContractBatchTransfer::Erc1155 {
            contract_address,
            from_address,
            to_address,
            token_ids,
            amounts,
            additional_data,
        } => {
            let to_address = address_from_str(&to_address)?;
            let from_address = address_from_str(&from_address)?;
            let token_ids = token_ids
                .iter()
                .map(|val| u256_from_str(val))
                .collect::<Result<Vec<U256>, _>>()?;
            let amounts = amounts
                .iter()
                .map(|val| u256_from_dec_str(val))
                .collect::<Result<Vec<U256>, _>>()?;
            let client = get_ethers_provider(web3api_url).await?;

            if let Some(key) = secret_key {
                let client = create_localwallet_client(polling_interval_ms, key, chain_id, client)?;

                let contract = Contract::new_erc1155(&contract_address, client)?;

                let call = contract.safe_batch_transfer_from(
                    from_address,
                    to_address,
                    token_ids,
                    amounts,
                    additional_data.into(),
                );
                let receipt = ContractCall::from(call).legacy(legacy).send().await?;
                Ok((Some(receipt), None))
            } else {
                let contract = Contract::new_erc1155(&contract_address, client)?;

                let call = contract.safe_batch_transfer_from(
                    from_address,
                    to_address,
                    token_ids,
                    amounts,
                    additional_data.into(),
                );
                let tx = ContractCall::from(call).legacy(legacy).get_tx();
                Ok((None, Some(tx)))
            }
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
    polling_interval_ms: u64,
) -> Result<EthersTransactionReceipt, EthError> {
    let (receipt, _) = broadcast_contract_batch_transfer_tx_common(
        details,
        network,
        Some(secret_key),
        web3api_url,
        polling_interval_ms,
    )
    .await?;
    receipt.ok_or_else(|| EthError::ContractSendError("No receipt".to_string()))
}

/// given the contract batch-transfer details, it'll construct
/// broadcast a corresponding transfer transaction.
/// If successful, it returns the typed transaction.
pub async fn construct_contract_batch_transfer_tx(
    details: ContractBatchTransfer,
    network: EthNetwork,
    web3api_url: &str,
) -> Result<TypedTransaction, EthError> {
    let (_, tx) =
        broadcast_contract_batch_transfer_tx_common(details, network, None, web3api_url, 0).await?;
    tx.ok_or_else(|| EthError::ContractSendError("No tx".to_string()))
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
    polling_interval_ms: u64,
) -> Result<EthersTransactionReceipt, EthError> {
    let (chain_id, legacy) = network.to_chain_params()?;

    let from_address = WalletCoinFunc {
        coin: WalletCoin::Ethereum {
            network: EthNetwork::Mainnet,
        },
    }
    .derive_address(secret_key.as_ref())
    .map_err(EthError::HdWrapError)?;
    let tx = construct_simple_eth_transfer_tx(&from_address, to_hex, amount, legacy, chain_id)?;
    let client = get_ethers_provider(web3api_url).await?;

    let client = create_localwallet_client(polling_interval_ms, secret_key, chain_id, client)?;

    let pending_tx = client
        .send_transaction(tx, None)
        .await
        .map_err(EthError::SendTxFail)?;
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
    polling_interval_ms: u64,
) -> Result<EthersTransactionReceipt, EthError> {
    let client = get_ethers_provider(web3api_url).await?;
    let provider = client.interval(Duration::from_millis(polling_interval_ms));
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
#[cfg(not(target_arch = "wasm32"))]
pub fn get_eth_balance_blocking(address: &str, web3api_url: &str) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    Ok(rt
        .block_on(get_eth_balance(address, web3api_url))?
        .to_string())
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
    polling_interval_ms: u64,
) -> Result<TransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_sign_eth_tx(
        to_hex,
        amount,
        network,
        secret_key,
        web3api_url,
        polling_interval_ms,
    ))?;
    Ok(result.into())
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
    polling_interval_ms: u64,
) -> Result<TransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_contract_approval_tx(
        approval_details,
        network,
        secret_key,
        web3api_url,
        polling_interval_ms,
    ))?;
    Ok(result.into())
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
    polling_interval_ms: u64,
) -> Result<TransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_contract_transfer_tx(
        transfer_details,
        network,
        secret_key,
        web3api_url,
        polling_interval_ms,
    ))?;
    Ok(result.into())
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
    polling_interval_ms: u64,
) -> Result<TransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_contract_batch_transfer_tx(
        batch_transfer_details,
        network,
        secret_key,
        web3api_url,
        polling_interval_ms,
    ))?;
    Ok(result.into())
}

/// broadcast a previously signed ethereum tx.
/// If successful, it returns the transaction hash/id.
/// (blocking; not compiled to wasm).
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_eth_signed_raw_tx_blocking(
    raw_tx: Vec<u8>,
    web3api_url: &str,
    polling_interval_ms: u64,
) -> Result<TransactionReceipt, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let result = rt.block_on(broadcast_eth_signed_raw_tx(
        raw_tx,
        web3api_url,
        polling_interval_ms,
    ))?;
    Ok(result.into())
}

#[inline]
pub fn address_from_str(address_str: &str) -> Result<Address, EthError> {
    Address::from_str(address_str).map_err(|_| EthError::HexConversion)
}

#[inline]
pub fn u256_from_str(u256_str: &str) -> Result<U256, EthError> {
    U256::from_str(u256_str).map_err(|_| EthError::HexConversion)
}

#[inline]
pub fn u256_from_dec_str(u256_str: &str) -> Result<U256, EthError> {
    U256::from_dec_str(u256_str).map_err(EthError::DecConversion)
}
