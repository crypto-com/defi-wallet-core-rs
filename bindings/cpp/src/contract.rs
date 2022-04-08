use crate::PrivateKey;
use anyhow::Result;
use common::get_contract_balance_blocking;
use common::node::ethereum;
use common::EthNetwork;
use defi_wallet_core_common as common;

/// Construct an Erc20 struct
fn new_erc20(contract_address: String, web3api_url: String, chain_id: u64) -> ffi::Erc20 {
    ffi::Erc20 {
        contract_address,
        web3api_url,
        inner_legacy: false,
        chain_id,
    }
}
impl ffi::Erc20 {
    /// Returns the decimal amount of tokens owned by `account_address`.
    fn balance_of(&self, account_address: String) -> Result<String> {
        let balance = get_contract_balance_blocking(
            &account_address,
            common::ContractBalance::Erc20 {
                contract_address: self.contract_address.clone(),
            },
            &self.web3api_url,
        )?;
        Ok(balance)
    }

    /// Returns the name of the token
    fn name(&self) -> Result<String> {
        let name = ethereum::erc20::get_name_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(name)
    }

    /// Returns the symbol of the token
    fn symbol(&self) -> Result<String> {
        let symbol =
            ethereum::erc20::get_symbol_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(symbol)
    }

    /// Returns the number of decimals the token uses
    fn decimals(&self) -> Result<u8> {
        let decimals =
            ethereum::erc20::get_decimals_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(decimals)
    }

    /// Makes a legacy transaction instead of an EIP-1559 one
    fn legacy(&mut self) -> Self {
        self.inner_legacy = true;
        self.clone()
    }

    /// Moves `amount_hex` tokens from the caller’s account to `to_address`.
    fn transfer(
        &self,
        to_address: String,
        amount_hex: String,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_transfer_tx_blocking(
            common::ContractTransfer::Erc20Transfer {
                contract_address: self.contract_address.clone(),
                to_address,
                amount_hex,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Moves `amount_hex` tokens from `from_address` to `to_address` using the allowance mechanism.
    fn transfer_from(
        &self,
        from_address: String,
        to_address: String,
        amount_hex: String,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_transfer_tx_blocking(
            common::ContractTransfer::Erc20TransferFrom {
                contract_address: self.contract_address.clone(),
                from_address,
                to_address,
                amount_hex,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Allows `approved_address` to withdraw from your account multiple times, up to the
    /// `amount_hex` amount.
    fn approve(
        &self,
        approved_address: String,
        amount_hex: String,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_approval_tx_blocking(
            common::ContractApproval::Erc20 {
                contract_address: self.contract_address.clone(),
                approved_address,
                amount_hex,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    fn allowance(&self, owner: String, spender: String) -> Result<String> {
        let allowance = ethereum::erc20::get_allowance_blocking(
            &self.contract_address,
            &owner,
            &spender,
            &self.web3api_url,
        )?;
        Ok(allowance.to_string())
    }

    /// Returns the amount of tokens in existence.
    fn total_supply(&self) -> Result<String> {
        let supply =
            ethereum::erc20::get_total_supply_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(supply.to_string())
    }
}

/// Construct an Erc721 struct
fn new_erc721(contract_address: String, web3api_url: String, chain_id: u64) -> ffi::Erc721 {
    ffi::Erc721 {
        contract_address,
        web3api_url,
        inner_legacy: false,
        chain_id,
    }
}
impl ffi::Erc721 {
    /// Returns the number of tokens in owner's `account_address`.
    fn balance_of(&self, account_address: String) -> Result<String> {
        let balance = get_contract_balance_blocking(
            &account_address,
            common::ContractBalance::Erc721 {
                contract_address: self.contract_address.clone(),
            },
            &self.web3api_url,
        )?;
        Ok(balance)
    }
    /// Returns the owner of the `token_id` token.
    fn owner_of(&self, token_id: String) -> Result<String> {
        let address = ethereum::erc721::get_token_owner_blocking(
            &self.contract_address,
            &token_id,
            &self.web3api_url,
        )?;
        Ok(format!("{:?}", address)) // we need the debug version of the address
    }

    /// Get the descriptive name for a collection of NFTs in this contract
    fn name(&self) -> Result<String> {
        let name = ethereum::erc721::get_name_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(name)
    }

    /// Get the abbreviated name for NFTs in this contract
    fn symbol(&self) -> Result<String> {
        let symbol =
            ethereum::erc721::get_symbol_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(symbol)
    }

    /// Get the distinct Uniform Resource Identifier (URI) for a given asset
    fn token_uri(&self, token_id: String) -> Result<String> {
        let token_uri = ethereum::erc721::get_token_uri_blocking(
            &self.contract_address,
            &token_id,
            &self.web3api_url,
        )?;
        Ok(token_uri)
    }

    /// Makes a legacy transaction instead of an EIP-1559 one
    fn legacy(&mut self) -> Self {
        self.inner_legacy = true;
        self.clone()
    }

    /// Transfers `token_id` token from `from_address` to `to_address`.
    fn transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_transfer_tx_blocking(
            common::ContractTransfer::Erc721TransferFrom {
                contract_address: self.contract_address.clone(),
                from_address,
                to_address,
                token_id,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Safely transfers `token_id` token from `from_address` to `to_address`.
    fn safe_transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_transfer_tx_blocking(
            common::ContractTransfer::Erc721SafeTransferFrom {
                contract_address: self.contract_address.clone(),
                from_address,
                to_address,
                token_id,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Safely transfers `token_id` token from `from_address` to `to_address` with
    /// `additional_data`.
    fn safe_transfer_from_with_data(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        additional_data: Vec<u8>,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_transfer_tx_blocking(
            common::ContractTransfer::Erc721SafeTransferFromWithAdditionalData {
                contract_address: self.contract_address.clone(),
                from_address,
                to_address,
                token_id,
                additional_data,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Gives permission to `approved_address` to transfer `token_id` token to another account.
    /// The approval is cleared when the token is transferred. Only a single account can be
    /// approved at a time, so approving the zero address clears previous approvals.
    fn approve(
        &self,
        approved_address: String,
        token_id: String,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_approval_tx_blocking(
            common::ContractApproval::Erc721Approve {
                contract_address: self.contract_address.clone(),
                approved_address,
                token_id,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Enable or disable approval for a third party `approved_address` to manage all of
    /// sender's assets
    fn set_approval_for_all(
        &self,
        approved_address: String,
        approved: bool,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_approval_tx_blocking(
            common::ContractApproval::Erc721SetApprovalForAll {
                contract_address: self.contract_address.clone(),
                approved_address,
                approved,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Get the approved address for a single NFT by `token_id`
    fn get_approved(&self, token_id: String) -> Result<String> {
        let address = ethereum::erc721::get_approved_blocking(
            &self.contract_address,
            &token_id,
            &self.web3api_url,
        )?;
        Ok(address.to_string())
    }

    /// Query if an address is an authorized `approved_address` for `owner`
    fn is_approved_for_all(&self, owner: String, approved_address: String) -> Result<bool> {
        let approved = ethereum::erc721::get_is_approved_for_all_blocking(
            &self.contract_address,
            &owner,
            &approved_address,
            &self.web3api_url,
        )?;
        Ok(approved)
    }

    /// Returns the total amount of tokens stored by the contract.
    ///
    /// From IERC721Enumerable, an optional extension of the standard ERC721
    fn total_supply(&self) -> Result<String> {
        let supply =
            ethereum::erc721::get_total_supply_blocking(&self.contract_address, &self.web3api_url)?;
        Ok(supply.to_string())
    }

    /// Returns a token ID at a given index of all the tokens stored by the contract. Use along
    /// with totalSupply to enumerate all tokens.
    ///
    /// From IERC721Enumerable, an optional extension of the standard ERC721
    fn token_by_index(&self, index: String) -> Result<String> {
        let token = ethereum::erc721::get_token_by_index_blocking(
            &self.contract_address,
            &index,
            &self.web3api_url,
        )?;
        Ok(token.to_string())
    }

    /// Returns a token ID owned by owner at a given index of its token list. Use along with
    /// balanceOf to enumerate all of owner's tokens.
    ///
    /// From IERC721Enumerable, an optional extension of the standard ERC721
    fn token_of_owner_by_index(&self, owner: String, index: String) -> Result<String> {
        let token = ethereum::erc721::get_token_of_owner_by_index_blocking(
            &self.contract_address,
            &owner,
            &index,
            &self.web3api_url,
        )?;
        Ok(token.to_string())
    }
}
/// Construct an Erc1155 struct
fn new_erc1155(contract_address: String, web3api_url: String, chain_id: u64) -> ffi::Erc1155 {
    ffi::Erc1155 {
        contract_address,
        web3api_url,
        inner_legacy: false,
        chain_id,
    }
}
impl ffi::Erc1155 {
    /// Returns the amount of tokens of `token_id` owned by `account_address`.
    fn balance_of(&self, account_address: String, token_id: String) -> Result<String> {
        let balance = get_contract_balance_blocking(
            &account_address,
            common::ContractBalance::Erc1155 {
                contract_address: self.contract_address.clone(),
                token_id,
            },
            &self.web3api_url,
        )?;
        Ok(balance)
    }

    /// Batched version of balance_of.
    /// Get the balance of multiple account/token pairs
    fn balance_of_batch(
        &self,
        account_addresses: Vec<String>,
        token_ids: Vec<String>,
    ) -> Result<Vec<String>> {
        let balance = ethereum::erc1155::get_balance_of_batch_blocking(
            &self.contract_address,
            account_addresses
                .iter()
                .map(|v| v.as_ref())
                .collect::<Vec<&str>>(),
            token_ids.iter().map(|v| v.as_ref()).collect::<Vec<&str>>(),
            &self.web3api_url,
        )?;
        Ok(balance
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>())
    }

    /// Get distinct Uniform Resource Identifier (URI) for a given token
    fn uri(&self, token_id: String) -> Result<String> {
        let uri = ethereum::erc1155::get_uri_blocking(
            &self.contract_address,
            &token_id,
            &self.web3api_url,
        )?;
        Ok(uri)
    }

    /// Makes a legacy transaction instead of an EIP-1559 one
    fn legacy(&mut self) -> Self {
        self.inner_legacy = true;
        self.clone()
    }

    /// Transfers `amount_hex` tokens of `token_id` from `from_address` to `to_address` with
    /// `additional_data`.
    fn safe_transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        amount_hex: String,
        additional_data: Vec<u8>,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_transfer_tx_blocking(
            common::ContractTransfer::Erc1155SafeTransferFrom {
                contract_address: self.contract_address.clone(),
                from_address,
                to_address,
                token_id,
                amount_hex,
                additional_data,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Batched version of safeTransferFrom.
    fn safe_batch_transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_ids: Vec<String>,
        hex_amounts: Vec<String>,
        additional_data: Vec<u8>,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_batch_transfer_tx_blocking(
            common::ContractBatchTransfer::Erc1155 {
                contract_address: self.contract_address.clone(),
                from_address,
                to_address,
                token_ids,
                hex_amounts,
                additional_data,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Enable or disable approval for a third party `approved_address` to manage all of
    /// sender's assets
    fn set_approval_for_all(
        &self,
        approved_address: String,
        approved: bool,
        private_key: &PrivateKey,
    ) -> Result<ffi::CronosTransactionReceiptRaw> {
        let receipt = common::broadcast_contract_approval_tx_blocking(
            common::ContractApproval::Erc1155 {
                contract_address: self.contract_address.clone(),
                approved_address,
                approved,
            },
            EthNetwork::Custom {
                chain_id: self.chain_id,
                legacy: self.inner_legacy,
            },
            private_key.key.clone(),
            &self.web3api_url,
        )?;
        Ok(receipt.into())
    }

    /// Query if an address is an authorized `approved_address` for `owner`
    fn is_approved_for_all(&self, owner: String, approved_address: String) -> Result<bool> {
        let approved = ethereum::erc1155::get_is_approved_for_all_blocking(
            &self.contract_address,
            &owner,
            &approved_address,
            &self.web3api_url,
        )?;
        Ok(approved)
    }
}

#[cxx::bridge(namespace = "org::defi_wallet_core")]
#[allow(clippy::too_many_arguments)]
mod ffi {

    #[derive(Clone)]
    pub struct Erc20 {
        contract_address: String,
        web3api_url: String,
        inner_legacy: bool,
        chain_id: u64,
    }

    #[derive(Clone)]
    pub struct Erc721 {
        contract_address: String,
        web3api_url: String,
        inner_legacy: bool,
        chain_id: u64,
    }

    #[derive(Clone)]
    pub struct Erc1155 {
        contract_address: String,
        web3api_url: String,
        inner_legacy: bool,
        chain_id: u64,
    }

    extern "C++" {
        include!("defi-wallet-core-cpp/src/lib.rs.h");
        type PrivateKey = crate::PrivateKey;
        type CronosTransactionReceiptRaw = crate::ffi::CronosTransactionReceiptRaw;
    }

    extern "Rust" {
        /// Construct an Erc20 struct
        fn new_erc20(address: String, web3api_url: String, chian_id: u64) -> Erc20;
        /// Returns the decimal amount of tokens owned by `account_address`.
        fn balance_of(self: &Erc20, account_address: String) -> Result<String>;
        /// Returns the name of the token
        fn name(self: &Erc20) -> Result<String>;
        /// Returns the symbol of the token
        fn symbol(self: &Erc20) -> Result<String>;
        /// Returns the number of decimals the token uses
        fn decimals(self: &Erc20) -> Result<u8>;
        /// Makes a legacy transaction instead of an EIP-1559 one
        fn legacy(self: &mut Erc20) -> Erc20;
        /// Moves `amount_hex` tokens from the caller’s account to `to_address`.
        fn transfer(
            self: &Erc20,
            to_address: String,
            amount_hex: String,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Moves `amount_hex` tokens from `from_address` to `to_address` using the allowance mechanism.
        fn transfer_from(
            self: &Erc20,
            from_address: String,
            to_address: String,
            amount_hex: String,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Allows `approved_address` to withdraw from your account multiple times, up to the
        /// `amount_hex` amount.
        fn approve(
            self: &Erc20,
            approved_address: String,
            amount_hex: String,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        fn allowance(self: &Erc20, owner: String, spender: String) -> Result<String>;
        /// Returns the amount of tokens in existence.
        fn total_supply(self: &Erc20) -> Result<String>;

        /// Construct an Erc721 struct
        fn new_erc721(address: String, web3api_url: String, chian_id: u64) -> Erc721;
        /// Returns the number of tokens in owner's `account_address`.
        fn balance_of(self: &Erc721, account_address: String) -> Result<String>;
        /// Returns the owner of the `token_id` token.
        fn owner_of(self: &Erc721, token_id: String) -> Result<String>;
        /// Get the descriptive name for a collection of NFTs in this contract
        fn name(self: &Erc721) -> Result<String>;
        /// Get the abbreviated name for NFTs in this contract
        fn symbol(self: &Erc721) -> Result<String>;
        /// Get the distinct Uniform Resource Identifier (URI) for a given asset
        fn token_uri(self: &Erc721, token_id: String) -> Result<String>;
        /// Makes a legacy transaction instead of an EIP-1559 one
        fn legacy(self: &mut Erc721) -> Erc721;
        /// Transfers `token_id` token from `from_address` to `to_address`.
        fn transfer_from(
            self: &Erc721,
            from_address: String,
            to_address: String,
            token_id: String,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Safely transfers `token_id` token from `from_address` to `to_address`.
        fn safe_transfer_from(
            self: &Erc721,
            from_address: String,
            to_address: String,
            token_id: String,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Safely transfers `token_id` token from `from_address` to `to_address` with
        /// `additional_data`.
        fn safe_transfer_from_with_data(
            self: &Erc721,
            from_address: String,
            to_address: String,
            token_id: String,
            additional_data: Vec<u8>,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Gives permission to `approved_address` to transfer `token_id` token to another account.
        /// The approval is cleared when the token is transferred. Only a single account can be
        /// approved at a time, so approving the zero address clears previous approvals.
        fn approve(
            self: &Erc721,
            approved_address: String,
            token_id: String,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Enable or disable approval for a third party `approved_address` to manage all of
        /// sender's assets
        fn set_approval_for_all(
            self: &Erc721,
            approved_address: String,
            approved: bool,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Get the approved address for a single NFT by `token_id`
        fn get_approved(self: &Erc721, token_id: String) -> Result<String>;
        /// Query if an address is an authorized `approved_address` for `owner`
        fn is_approved_for_all(
            self: &Erc721,
            owner: String,
            approved_address: String,
        ) -> Result<bool>;
        /// Returns the total amount of tokens stored by the contract.
        ///
        /// From IERC721Enumerable, an optional extension of the standard ERC721
        fn total_supply(self: &Erc721) -> Result<String>;
        /// Returns a token ID at a given index of all the tokens stored by the contract. Use along
        /// with totalSupply to enumerate all tokens.
        ///
        /// From IERC721Enumerable, an optional extension of the standard ERC721
        fn token_by_index(self: &Erc721, index: String) -> Result<String>;
        /// Returns a token ID owned by owner at a given index of its token list. Use along with
        /// balanceOf to enumerate all of owner's tokens.
        ///
        /// From IERC721Enumerable, an optional extension of the standard ERC721
        fn token_of_owner_by_index(self: &Erc721, owner: String, index: String) -> Result<String>;

        /// Construct an Erc1155 struct
        fn new_erc1155(address: String, web3api_url: String, chian_id: u64) -> Erc1155;
        /// Returns the amount of tokens of `token_id` owned by `account_address`.
        fn balance_of(self: &Erc1155, account_address: String, token_id: String) -> Result<String>;
        /// Batched version of balance_of.
        /// Get the balance of multiple account/token pairs
        fn balance_of_batch(
            self: &Erc1155,
            account_addresses: Vec<String>,
            token_ids: Vec<String>,
        ) -> Result<Vec<String>>;
        /// Get distinct Uniform Resource Identifier (URI) for a given token
        fn uri(self: &Erc1155, token_id: String) -> Result<String>;
        /// Makes a legacy transaction instead of an EIP-1559 one
        fn legacy(self: &mut Erc1155) -> Erc1155;
        /// Transfers `amount_hex` tokens of `token_id` from `from_address` to `to_address` with
        /// `additional_data`.
        fn safe_transfer_from(
            self: &Erc1155,
            from_address: String,
            to_address: String,
            token_id: String,
            amount_hex: String,
            additional_data: Vec<u8>,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Batched version of safeTransferFrom.
        fn safe_batch_transfer_from(
            self: &Erc1155,
            from_address: String,
            to_address: String,
            token_ids: Vec<String>,
            hex_amounts: Vec<String>,
            additional_data: Vec<u8>,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Enable or disable approval for a third party `approved_address` to manage all of
        /// sender's assets
        fn set_approval_for_all(
            self: &Erc1155,
            approved_address: String,
            approved: bool,
            private_key: &PrivateKey,
        ) -> Result<CronosTransactionReceiptRaw>;
        /// Query if an address is an authorized `approved_address` for `owner`
        fn is_approved_for_all(
            self: &Erc1155,
            owner: String,
            approved_address: String,
        ) -> Result<bool>;
    }
}
