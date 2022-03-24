use anyhow::Result;
use common::get_contract_balance_blocking;
use common::node::ethereum;
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
fn new_erc20(address: String, web3api_url: String) -> ffi::Erc20 {
    ffi::Erc20 {
        address,
        web3api_url,
    }
}
impl ffi::Erc20 {
    /// Returns the name of the token
    fn name(&self) -> Result<String> {
        let name = ethereum::erc20::get_name_blocking(&self.address, &self.web3api_url)?;
        Ok(name)
    }

    /// Returns the symbol of the token
    fn symbol(&self) -> Result<String> {
        let symbol = ethereum::erc20::get_symbol_blocking(&self.address, &self.web3api_url)?;
        Ok(symbol)
    }

    /// Returns the number of decimals the token uses
    fn decimals(&self) -> Result<u8> {
        let decimals = ethereum::erc20::get_decimals_blocking(&self.address, &self.web3api_url)?;
        Ok(decimals)
    }
}

/// Construct an Erc721 struct
fn new_erc721(address: String, web3api_url: String) -> ffi::Erc721 {
    ffi::Erc721 {
        address,
        web3api_url,
    }
}
impl ffi::Erc721 {
    /// Get the descriptive name for a collection of NFTs in this contract
    fn name(&self) -> Result<String> {
        let name = ethereum::erc721::get_name_blocking(&self.address, &self.web3api_url)?;
        Ok(name)
    }

    /// Get the abbreviated name for NFTs in this contract
    fn symbol(&self) -> Result<String> {
        let symbol = ethereum::erc721::get_symbol_blocking(&self.address, &self.web3api_url)?;
        Ok(symbol)
    }

    /// Get the distinct Uniform Resource Identifier (URI) for a given asset
    fn token_uri(&self, token_id: String) -> Result<String> {
        let token_uri =
            ethereum::erc721::get_token_uri_blocking(&self.address, &token_id, &self.web3api_url)?;
        Ok(token_uri)
    }
}
/// Construct an Erc1155 struct
fn new_erc1155(address: String, web3api_url: String) -> ffi::Erc1155 {
    ffi::Erc1155 {
        address,
        web3api_url,
    }
}
impl ffi::Erc1155 {
    /// Get distinct Uniform Resource Identifier (URI) for a given token
    fn uri(&self, token_id: String) -> Result<String> {
        let uri = ethereum::erc1155::get_uri_blocking(&self.address, &token_id, &self.web3api_url)?;
        Ok(uri)
    }
}

#[cxx::bridge(namespace = "org::defi_wallet_core")]
mod ffi {

    pub struct Erc20 {
        address: String,
        web3api_url: String,
    }

    pub struct Erc721 {
        address: String,
        web3api_url: String,
    }

    pub struct Erc1155 {
        address: String,
        web3api_url: String,
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

        fn new_erc20(address: String, web3api_url: String) -> Erc20;
        fn name(self: &Erc20) -> Result<String>;
        fn symbol(self: &Erc20) -> Result<String>;
        fn decimals(self: &Erc20) -> Result<u8>;

        fn new_erc721(address: String, web3api_url: String) -> Erc721;
        fn name(self: &Erc721) -> Result<String>;
        fn symbol(self: &Erc721) -> Result<String>;
        fn token_uri(self: &Erc721, token_id: String) -> Result<String>;

        fn new_erc1155(address: String, web3api_url: String) -> Erc1155;
        fn uri(self: &Erc1155, token_id: String) -> Result<String>;
    }
}
