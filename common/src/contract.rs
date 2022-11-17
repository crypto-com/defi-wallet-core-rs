use crate::EthError;
#[cfg(feature = "abi-contract")]
use crate::{abi::EthAbiToken, EthAbiTokenBind};
use ethers::abi::Detokenize;
#[cfg(feature = "abi-contract")]
use ethers::abi::Token;
use ethers::contract::builders;
use ethers::prelude::{abigen, Middleware, TransactionReceipt, U256};
use ethers::types::transaction::eip2718::TypedTransaction;
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
#[cfg(feature = "erc4907")]
abigen!(
    Erc4907Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc4907-abi.json"
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

    /// Construct an ERC907 contract
    #[cfg(feature = "erc4907")]
    pub fn new_erc4907<M>(contract_address: &str, client: M) -> Result<Erc4907Contract<M>, EthError>
    where
        M: Middleware,
    {
        let contract_address = address_from_str(contract_address)?;
        Ok(Erc4907Contract::new(contract_address, Arc::new(client)))
    }

    /// Construct a dynamic contract based on the provided address,
    /// ABI string, and client middleware.
    #[cfg(feature = "abi-contract")]
    pub fn new_dynamic<M>(
        contract_address: &str,
        abi_json: &str,
        client: M,
    ) -> Result<DynamicContract<M>, EthError>
    where
        M: Middleware,
    {
        DynamicContract::new(contract_address, abi_json, client)
    }
}

/// A dynamic contract that can be constructed from an ABI string
/// (function calls are checked during runtime)
#[cfg(feature = "abi-contract")]
pub struct DynamicContract<M: Middleware>(ethers::contract::Contract<M>);

#[cfg(feature = "abi-contract")]
impl<M: Middleware> DynamicContract<M> {
    /// Construct a dynamic contract
    ///
    /// # Arguments
    /// * `contract_address` - The address of the contract
    /// * `abi_json` - The raw ABI of the contract (string loaded e.g. from a file)
    /// * `client` - The client middleware
    pub fn new(
        contract_address: &str,
        abi_json: &str,
        client: M,
    ) -> Result<DynamicContract<M>, EthError> {
        let contract_address = address_from_str(contract_address)?;
        let abi: ethers::abi::Abi = serde_json::from_str(abi_json).map_err(EthError::JsonError)?;
        let contract = ethers::contract::Contract::new(contract_address, abi, Arc::new(client));
        Ok(Self(contract))
    }

    /// Creates a contract call builder based on the provided method name and
    /// arguments. The method name must be present in the ABI.
    /// The arguments are checked against the ABI at runtime
    /// -- i.e. the caller is responsible for ensuring that the arguments
    /// are in the correct order and that the types are correct.
    ///
    /// The returned contract call then be called using `send`
    /// (if a transaction is required) or `call` (if it is a static function).
    ///
    /// # Arguments
    /// * `method_name` - The name of the method to call
    /// * `params` - The arguments to pass to the method (must be in the correct order)
    pub fn function_call<D: Detokenize>(
        &self,
        method_name: &str,
        params: Vec<EthAbiTokenBind>,
    ) -> Result<ContractCall<M, D>, EthError> {
        let tokens = params
            .iter()
            .flat_map(EthAbiToken::try_from)
            .map(|x| Token::try_from(&x))
            .collect::<Result<Vec<Token>, _>>()?;
        let method = self
            .0
            .method::<_, D>(method_name, Token::Tuple(tokens))
            .map_err(EthError::DynamicAbiError)?;
        Ok(method.into())
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

impl<M, D> ContractCall<M, D>
where
    // S: Signer,
    M: Middleware,
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

    /// Returns the raw transaction request
    pub fn get_tx(&self) -> TypedTransaction {
        self.contract_call.tx.clone()
    }

    /// Signs and broadcasts the provided transaction
    pub async fn send(&self) -> Result<TransactionReceipt, EthError> {
        let pending_tx = self
            .contract_call
            .send()
            .await
            .map_err(|e| EthError::ContractSendError(e.to_string()))?
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
            .map_err(|e| EthError::ContractSendError(e.to_string()))
    }

    /// Queries the blockchain via an eth_call for the provided transaction.
    pub async fn call(&self) -> Result<D, EthError> {
        self.contract_call
            .call()
            .await
            .map_err(|e| EthError::ContractCallError(e.to_string()))
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
