use crate::PrivateKey;
use anyhow::Result;
use common::get_contract_balance_blocking;
use common::node::ethereum;
use common::EthNetwork;
use defi_wallet_core_common as common;

/// Wrapper of `ContractBalance`
pub struct ContractBalance(common::ContractBalance);

/// Contruct a boxed erc20 ContractBalance struct
pub fn erc20_balance(contract_address: String) -> Box<ContractBalance> {
    Box::new(ContractBalance(common::ContractBalance::Erc20 {
        contract_address,
    }))
}

/// Contruct a boxed erc721 ContractBalance struct
pub fn erc721_balance(contract_address: String) -> Box<ContractBalance> {
    Box::new(ContractBalance(common::ContractBalance::Erc721 {
        contract_address,
    }))
}

/// Contruct a boxed erc1155 ContractBalance struct
pub fn erc1155_balance(contract_address: String, token_id: String) -> Box<ContractBalance> {
    Box::new(ContractBalance(common::ContractBalance::Erc1155 {
        contract_address,
        token_id,
    }))
}

/// get contract balance from cronos node
pub fn get_contract_balance(
    address: &str,
    contract_details: &ContractBalance,
    api_url: &str,
) -> Result<String> {
    let res = get_contract_balance_blocking(address, contract_details.0.clone(), api_url)?;
    Ok(res)
}

pub struct ContractOwner(common::ContractOwner);

/// Contruct a boxed erc721 ContractOwner struct
pub fn erc721_owner(contract_address: String, token_id: String) -> Box<ContractOwner> {
    Box::new(ContractOwner(common::ContractOwner::Erc721 {
        contract_address,
        token_id,
    }))
}

/// get contract owner from cronos node
pub fn get_token_owner(contract_owner: &ContractOwner, api_url: &str) -> Result<String> {
    let res = common::get_token_owner_blocking(contract_owner.0.clone(), api_url)?;
    Ok(format!("{:?}", res)) // we need the debug version of the address
}

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

    fn transfer(
        &self,
        to_address: String,
        amount_hex: String,
        private_key: &PrivateKey,
    ) -> Result<String> {
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
        Ok(receipt)
    }

    fn transfer_from(
        &self,
        from_address: String,
        to_address: String,
        amount_hex: String,
        private_key: &PrivateKey,
    ) -> Result<String> {
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
        Ok(receipt)
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

    fn transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        private_key: &PrivateKey,
    ) -> Result<String> {
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
        Ok(receipt)
    }

    fn safe_transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        private_key: &PrivateKey,
    ) -> Result<String> {
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
        Ok(receipt)
    }
    fn safe_transfer_from_with_data(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        additional_data: Vec<u8>,
        private_key: &PrivateKey,
    ) -> Result<String> {
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
        Ok(receipt)
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

    fn safe_transfer_from(
        &self,
        from_address: String,
        to_address: String,
        token_id: String,
        amount_hex: String,
        additional_data: Vec<u8>,
        private_key: &PrivateKey,
    ) -> Result<String> {
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
        Ok(receipt)
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
    }

    extern "Rust" {

        type ContractBalance;
        fn erc20_balance(contract_address: String) -> Box<ContractBalance>;
        fn erc721_balance(contract_address: String) -> Box<ContractBalance>;
        fn erc1155_balance(contract_address: String, token_id: String) -> Box<ContractBalance>;
        fn get_contract_balance(
            address: &str,
            contract_details: &ContractBalance,
            api_url: &str,
        ) -> Result<String>;

        type ContractOwner;
        fn erc721_owner(contract_address: String, token_id: String) -> Box<ContractOwner>;
        fn get_token_owner(contract_owner: &ContractOwner, api_url: &str) -> Result<String>;

        fn new_erc20(address: String, web3api_url: String, chian_id: u64) -> Erc20;
        fn name(self: &Erc20) -> Result<String>;
        fn symbol(self: &Erc20) -> Result<String>;
        fn decimals(self: &Erc20) -> Result<u8>;
        fn legacy(self: &mut Erc20) -> Erc20;
        fn transfer(
            self: &Erc20,
            to_address: String,
            amount_hex: String,
            private_key: &PrivateKey,
        ) -> Result<String>;
        fn transfer_from(
            self: &Erc20,
            from_address: String,
            to_address: String,
            amount_hex: String,
            private_key: &PrivateKey,
        ) -> Result<String>;

        fn new_erc721(address: String, web3api_url: String, chian_id: u64) -> Erc721;
        fn name(self: &Erc721) -> Result<String>;
        fn symbol(self: &Erc721) -> Result<String>;
        fn token_uri(self: &Erc721, token_id: String) -> Result<String>;
        fn legacy(self: &mut Erc721) -> Erc721;
        fn transfer_from(
            self: &Erc721,
            from_address: String,
            to_address: String,
            token_id: String,
            private_key: &PrivateKey,
        ) -> Result<String>;
        fn safe_transfer_from(
            self: &Erc721,
            from_address: String,
            to_address: String,
            token_id: String,
            private_key: &PrivateKey,
        ) -> Result<String>;

        fn safe_transfer_from_with_data(
            self: &Erc721,
            from_address: String,
            to_address: String,
            token_id: String,
            additional_data: Vec<u8>,
            private_key: &PrivateKey,
        ) -> Result<String>;

        fn new_erc1155(address: String, web3api_url: String, chian_id: u64) -> Erc1155;
        fn uri(self: &Erc1155, token_id: String) -> Result<String>;
        fn legacy(self: &mut Erc1155) -> Erc1155;
        fn safe_transfer_from(
            self: &Erc1155,
            from_address: String,
            to_address: String,
            token_id: String,
            amount_hex: String,
            additional_data: Vec<u8>,
            private_key: &PrivateKey,
        ) -> Result<String>;

    }
}
