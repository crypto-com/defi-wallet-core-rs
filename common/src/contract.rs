use crate::EthError;
use ethers::abi::Detokenize;
use ethers::contract::builders;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{
    abigen, Http, Middleware, Provider, SignerMiddleware, TransactionReceipt, Wallet, U256,
};
use std::sync::Arc;

abigen!(
    Erc20Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc20-abi.json"
);
abigen!(
    Erc721Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc721-abi.json"
);
abigen!(
    Erc1155Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc1155-abi.json"
);

///  Wrapper of ERC20, ERC721 and ERC1155 contracts
///  TODO Put utils.rs contract related functions under it
pub struct Contract;

use crate::node::ethereum::utils::address_from_str;

impl Contract {
    /// Construct an ERC20 contract
    pub fn new_erc20<M>(contract_address: &str, client: M) -> Result<Erc20Contract<M>, EthError>
    where
        M: Middleware,
    {
        let contract_address = address_from_str(contract_address)?;
        Ok(Erc20Contract::new(contract_address, Arc::new(client)))
    }

    /// Construct an ERC721 contract
    pub fn new_erc721<M>(contract_address: &str, client: M) -> Result<Erc721Contract<M>, EthError>
    where
        M: Middleware,
    {
        let contract_address = address_from_str(contract_address)?;
        Ok(Erc721Contract::new(contract_address, Arc::new(client)))
    }

    /// Construct an ERC1155 contract
    pub fn new_erc1155<M>(contract_address: &str, client: M) -> Result<Erc1155Contract<M>, EthError>
    where
        M: Middleware,
    {
        let contract_address = address_from_str(contract_address)?;
        Ok(Erc1155Contract::new(contract_address, Arc::new(client)))
    }
}

/// Wrapper of ContractCall
pub struct ContractCall<M, D>
where
    M: Middleware,
    D: Detokenize,
{
    contract_call: builders::ContractCall<M, D>,
}

impl<D> ContractCall<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, D>
where
    D: Detokenize,
{
    /// Uses a Legacy transaction instead of an EIP-1559 one to execute the call. If legacy is
    /// true, it will use legacy transaction, else it will use EIP-1559 transaction
    ///
    /// !!! Please notice: This function can only be called once. If the ContractCall is
    /// converted into the legacy one, you can not convert it back. !!!
    ///
    pub fn legacy(mut self, legacy: bool) -> Self {
        if legacy {
            self.contract_call = self.contract_call.legacy();
        }
        self
    }
    /// Signs and broadcasts the provided transaction
    pub async fn send(&self) -> Result<TransactionReceipt, EthError> {
        let pending_tx = self
            .contract_call
            .send()
            .await
            .map_err(EthError::ContractSendError)?
            .await;
        let tx_receipt = pending_tx
            .map_err(EthError::BroadcastTxFail)?
            .ok_or(EthError::MempoolDrop)?;
        Ok(tx_receipt)
    }
    // TODO Returns the estimated gas cost for the underlying transaction to be executed
    pub async fn estimate_gas(&self) -> Result<U256, EthError> {
        self.contract_call
            .estimate_gas()
            .await
            .map_err(EthError::ContractSendError)
    }
}

impl<M, D> From<builders::ContractCall<M, D>> for ContractCall<M, D>
where
    M: Middleware,
    D: Detokenize,
{
    fn from(contract_call: builders::ContractCall<M, D>) -> Self {
        Self { contract_call }
    }
}

impl<D> ContractCall<Provider<Http>, D>
where
    D: Detokenize,
{
    /// Queries the blockchain via an eth_call for the provided transaction.
    pub async fn call(&self) -> Result<D, EthError> {
        self.contract_call
            .call()
            .await
            .map_err(EthError::ContractCallError)
    }
    // TODO Returns the estimated gas cost for the underlying transaction to be executed
    pub async fn estimate_gas(&self) -> Result<U256, EthError> {
        self.contract_call
            .estimate_gas()
            .await
            .map_err(EthError::ContractCallError)
    }
}
