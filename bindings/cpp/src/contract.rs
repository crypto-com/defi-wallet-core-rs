use anyhow::Result;

/// Wrapper of `ContractBalance`
pub struct ContractBalance(defi_wallet_core_common::ContractBalance);

/// Contruct a boxed erc20 ContractBalance struct
pub fn erc20_balance(contract_address: String) -> Box<ContractBalance> {
    Box::new(ContractBalance(
        defi_wallet_core_common::ContractBalance::Erc20 { contract_address },
    ))
}

/// Contruct a boxed erc20 ContractBalance struct
pub fn erc721_balance(contract_address: String) -> Box<ContractBalance> {
    Box::new(ContractBalance(
        defi_wallet_core_common::ContractBalance::Erc721 { contract_address },
    ))
}

/// Contruct a boxed erc20 ContractBalance struct
pub fn erc1155_balance(contract_address: String, token_id: String) -> Box<ContractBalance> {
    Box::new(ContractBalance(
        defi_wallet_core_common::ContractBalance::Erc1155 {
            contract_address,
            token_id,
        },
    ))
}

/// get contract balance from cronos node
pub fn get_contract_balance(
    address: &str,
    contract_details: &ContractBalance,
    api_url: &str,
) -> Result<String> {
    let res = defi_wallet_core_common::get_contract_balance_blocking(
        address,
        contract_details.0.clone(),
        api_url,
    )?;
    Ok(res)
}
