use crate::PrivateKey;
use defi_wallet_core_common::{
    broadcast_contract_approval_tx, broadcast_contract_batch_transfer_tx,
    broadcast_contract_transfer_tx, broadcast_sign_eth_tx, get_contract_balance, get_eth_balance,
    ContractApproval, ContractBalance, ContractBatchTransfer, ContractTransfer, EthAmount,
    EthNetwork,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// return the account's balance formatted as ether decimals
#[wasm_bindgen]
pub async fn query_account_eth_balance(
    web3_api_url: String,
    address: String,
) -> Result<JsValue, JsValue> {
    let balance = get_eth_balance(&address, &web3_api_url)
        .await
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_str(&balance))
}

/// the token contract type
#[wasm_bindgen]
pub enum ContractType {
    Erc20,
    Erc721,
    Erc1155,
}

/// return the account's token contract balance formatted as hexadecimals
#[wasm_bindgen]
pub async fn query_account_contract_token_balance(
    web3_api_url: String,
    address: String,
    contract_address: String,
    contract_type: ContractType,
    token_id: Option<String>,
) -> Result<JsValue, JsValue> {
    let details = match (contract_type, token_id) {
        (ContractType::Erc20, _) => Ok(ContractBalance::Erc20 { contract_address }),
        (ContractType::Erc721, _) => Ok(ContractBalance::Erc721 { contract_address }),
        (ContractType::Erc1155, Some(token_id)) => Ok(ContractBalance::Erc1155 {
            contract_address,
            token_id,
        }),
        (ContractType::Erc1155, None) => Err(JsValue::from_str("missing token id")),
    }?;
    let balance = get_contract_balance(&address, details, &web3_api_url)
        .await
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_str(&balance.to_string()))
}

/// construct, sign and broadcast a plain transfer of eth/native token
#[wasm_bindgen]
pub async fn broadcast_transfer_eth(
    web3_api_url: String,
    to_address_hex: String,
    eth_amount_decimal: String,
    chain_id: u64,
    private_key: PrivateKey,
) -> Result<JsValue, JsValue> {
    let receipt = broadcast_sign_eth_tx(
        &to_address_hex,
        EthAmount::EthDecimal {
            amount: eth_amount_decimal,
        },
        EthNetwork::Custom {
            chain_id,
            legacy: false,
        },
        private_key.key,
        &web3_api_url,
    )
    .await
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_serde(&receipt).map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}

/// details needed for contract approval transaction
#[wasm_bindgen]
pub struct ContractApprovalDetails {
    approved_address: String,
    contract_address: String,
    contract_type: ContractType,
    amount_hex: Option<String>,
    token_id: Option<String>,
    approved: Option<bool>,
}

#[wasm_bindgen]
impl ContractApprovalDetails {
    /// constructs arguments for ERC-20 function approve
    #[wasm_bindgen]
    pub fn build_erc20_approve(
        contract_address: String,
        spender_address: String,
        amount_hex: String,
    ) -> Self {
        Self {
            approved_address: spender_address,
            contract_address,
            contract_type: ContractType::Erc20,
            amount_hex: Some(amount_hex),
            token_id: None,
            approved: None,
        }
    }

    /// constructs arguments for ERC-721 function approve
    #[wasm_bindgen]
    pub fn build_erc721_approve(
        contract_address: String,
        approved_address: String,
        token_id: String,
    ) -> Self {
        Self {
            approved_address,
            contract_address,
            contract_type: ContractType::Erc721,
            amount_hex: None,
            token_id: Some(token_id),
            approved: None,
        }
    }

    /// constructs arguments for ERC-721 function setApprovalForAll
    #[wasm_bindgen]
    pub fn build_erc721_set_approval_for_all(
        contract_address: String,
        operator_address: String,
        approved: bool,
    ) -> Self {
        Self {
            approved_address: operator_address,
            contract_address,
            contract_type: ContractType::Erc721,
            amount_hex: None,
            token_id: None,
            approved: Some(approved),
        }
    }

    /// constructs arguments for ERC-1155 function setApprovalForAll
    #[wasm_bindgen]
    pub fn build_erc1155_set_approval_for_all(
        contract_address: String,
        operator_address: String,
        approved: bool,
    ) -> Self {
        Self {
            approved_address: operator_address,
            contract_address,
            contract_type: ContractType::Erc1155,
            amount_hex: None,
            token_id: None,
            approved: Some(approved),
        }
    }
}

impl TryFrom<ContractApprovalDetails> for ContractApproval {
    type Error = JsValue;

    fn try_from(details: ContractApprovalDetails) -> Result<Self, Self::Error> {
        match (
            details.contract_type,
            details.amount_hex,
            details.token_id,
            details.approved,
        ) {
            (ContractType::Erc20, Some(amount_hex), _, _) => Ok(Self::Erc20 {
                contract_address: details.contract_address,
                approved_address: details.approved_address,
                amount_hex,
            }),
            (ContractType::Erc721, _, Some(token_id), _) => Ok(Self::Erc721Approve {
                contract_address: details.contract_address,
                approved_address: details.approved_address,
                token_id,
            }),
            (ContractType::Erc721, _, _, Some(approved)) => Ok(Self::Erc721SetApprovalForAll {
                contract_address: details.contract_address,
                approved_address: details.approved_address,
                approved,
            }),
            (ContractType::Erc1155, _, _, Some(approved)) => Ok(Self::Erc1155 {
                contract_address: details.contract_address,
                approved_address: details.approved_address,
                approved,
            }),
            (ContractType::Erc20, None, _, _) => Err(JsValue::from_str("missing amount")),
            (ContractType::Erc721, _, None, None) => {
                Err(JsValue::from_str("missing token id or approved"))
            }
            (ContractType::Erc1155, _, _, None) => Err(JsValue::from_str("missing approved")),
        }
    }
}

/// details needed for contract transfer transaction
#[wasm_bindgen]
pub struct ContractTransferDetails {
    is_safe: bool,
    to_address: String,
    contract_address: String,
    contract_type: ContractType,
    from_address: Option<String>,
    token_id: Option<String>,
    amount_hex: Option<String>,
    additional_data: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl ContractTransferDetails {
    /// constructs arguments for ERC-20 function transfer
    #[wasm_bindgen]
    pub fn build_erc20_transfer(
        contract_address: String,
        to_address: String,
        amount_hex: String,
    ) -> Self {
        Self {
            is_safe: false,
            to_address,
            contract_address,
            contract_type: ContractType::Erc20,
            from_address: None,
            token_id: None,
            amount_hex: Some(amount_hex),
            additional_data: None,
        }
    }

    /// constructs arguments for ERC-20 function transferFrom
    #[wasm_bindgen]
    pub fn build_erc20_transfer_from(
        contract_address: String,
        from_address: String,
        to_address: String,
        amount_hex: String,
    ) -> Self {
        Self {
            is_safe: false,
            to_address,
            contract_address,
            contract_type: ContractType::Erc20,
            from_address: Some(from_address),
            token_id: None,
            amount_hex: Some(amount_hex),
            additional_data: None,
        }
    }

    /// constructs arguments for ERC-721 function transferFrom
    #[wasm_bindgen]
    pub fn build_erc721_transfer_from(
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
    ) -> Self {
        Self {
            is_safe: false,
            to_address,
            contract_address,
            contract_type: ContractType::Erc721,
            from_address: Some(from_address),
            token_id: Some(token_id),
            amount_hex: None,
            additional_data: None,
        }
    }

    /// constructs arguments for ERC-721 function safeTransferFrom (no additional data)
    #[wasm_bindgen]
    pub fn build_erc721_safe_transfer_from(
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
    ) -> Self {
        Self {
            is_safe: true,
            to_address,
            contract_address,
            contract_type: ContractType::Erc721,
            from_address: Some(from_address),
            token_id: Some(token_id),
            amount_hex: None,
            additional_data: None,
        }
    }

    /// constructs arguments for ERC-721 function safeTransferFrom with argument additional data
    #[wasm_bindgen]
    pub fn build_erc721_safe_transfer_from_with_additional_data(
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
        additional_data: Vec<u8>,
    ) -> Self {
        Self {
            is_safe: true,
            to_address,
            contract_address,
            contract_type: ContractType::Erc721,
            from_address: Some(from_address),
            token_id: Some(token_id),
            amount_hex: None,
            additional_data: Some(additional_data),
        }
    }

    /// constructs arguments for ERC-1155 function safeTransferFrom
    #[wasm_bindgen]
    pub fn build_erc1155_safe_transfer_from(
        contract_address: String,
        from_address: String,
        to_address: String,
        token_id: String,
        amount_hex: String,
        additional_data: Vec<u8>,
    ) -> Self {
        Self {
            is_safe: true,
            to_address,
            contract_address,
            contract_type: ContractType::Erc1155,
            from_address: Some(from_address),
            token_id: Some(token_id),
            amount_hex: Some(amount_hex),
            additional_data: Some(additional_data),
        }
    }
}

impl TryFrom<ContractTransferDetails> for ContractTransfer {
    type Error = JsValue;

    fn try_from(details: ContractTransferDetails) -> Result<Self, Self::Error> {
        match (
            details.contract_type,
            details.is_safe,
            details.from_address,
            details.token_id,
            details.amount_hex,
            details.additional_data,
        ) {
            (ContractType::Erc20, _, None, _, Some(amount_hex), _) => {
                Ok(ContractTransfer::Erc20Transfer {
                    contract_address: details.contract_address,
                    to_address: details.to_address,
                    amount_hex,
                })
            }
            (ContractType::Erc20, _, Some(from_address), _, Some(amount_hex), _) => {
                Ok(ContractTransfer::Erc20TransferFrom {
                    contract_address: details.contract_address,
                    from_address,
                    to_address: details.to_address,
                    amount_hex,
                })
            }
            (ContractType::Erc721, false, Some(from_address), Some(token_id), _, _) => {
                Ok(ContractTransfer::Erc721TransferFrom {
                    contract_address: details.contract_address,
                    from_address,
                    to_address: details.to_address,
                    token_id,
                })
            }
            (ContractType::Erc721, true, Some(from_address), Some(token_id), _, None) => {
                Ok(ContractTransfer::Erc721SafeTransferFrom {
                    contract_address: details.contract_address,
                    from_address,
                    to_address: details.to_address,
                    token_id,
                })
            }
            (
                ContractType::Erc721,
                true,
                Some(from_address),
                Some(token_id),
                _,
                Some(additional_data),
            ) => Ok(ContractTransfer::Erc721SafeTransferFromWithAdditionalData {
                contract_address: details.contract_address,
                from_address,
                to_address: details.to_address,
                token_id,
                additional_data: additional_data,
            }),
            (
                ContractType::Erc1155,
                _,
                Some(from_address),
                Some(token_id),
                Some(amount_hex),
                additional_data,
            ) => Ok(ContractTransfer::Erc1155SafeTransferFrom {
                contract_address: details.contract_address,
                from_address,
                to_address: details.to_address,
                token_id,
                amount_hex,
                additional_data: additional_data.unwrap_or_else(|| vec![]),
            }),
            (ContractType::Erc1155, _, None, _, _, _)
            | (ContractType::Erc721, _, None, _, _, _) => {
                Err(JsValue::from_str("missing from address"))
            }
            (ContractType::Erc1155, _, _, None, _, _)
            | (ContractType::Erc721, _, _, None, _, _) => {
                Err(JsValue::from_str("missing token_id"))
            }
            (ContractType::Erc1155, _, _, _, None, _) | (ContractType::Erc20, _, _, _, None, _) => {
                Err(JsValue::from_str("missing amount"))
            }
        }
    }
}

/// details needed for contract batch-transfer transaction
/// Fix `amount`, `token_ids` or `additional_data` to optional if any of these
/// fields is not necessary for other batch-tranfer functions.
#[wasm_bindgen]
pub struct ContractBatchTransferDetails {
    from_address: String,
    to_address: String,
    contract_address: String,
    contract_type: ContractType,
    hex_amounts: Vec<String>,
    token_ids: Vec<String>,
    additional_data: Vec<u8>,
}

#[wasm_bindgen]
impl ContractBatchTransferDetails {
    /// constructs arguments for ERC-1155 function safeBatchTransferFrom
    #[wasm_bindgen]
    pub fn build_erc1155_safe_batch_transfer_from(
        contract_address: String,
        from_address: String,
        to_address: String,
        // Original item type of vector must be TokenAmount.
        token_amounts: Vec<JsValue>,
        additional_data: Vec<u8>,
    ) -> Result<ContractBatchTransferDetails, JsValue> {
        let len = token_amounts.len();
        let mut token_ids = Vec::with_capacity(len);
        let mut hex_amounts = Vec::with_capacity(len);
        for item in token_amounts {
            let token_amount: TokenAmount = item.try_into()?;
            token_ids.push(token_amount.token_id);
            hex_amounts.push(token_amount.hex_amount);
        }
        Ok(Self {
            from_address,
            to_address,
            contract_address,
            contract_type: ContractType::Erc1155,
            hex_amounts,
            token_ids,
            additional_data,
        })
    }
}

impl TryFrom<ContractBatchTransferDetails> for ContractBatchTransfer {
    type Error = JsValue;

    fn try_from(details: ContractBatchTransferDetails) -> Result<Self, Self::Error> {
        match details.contract_type {
            ContractType::Erc1155 => Ok(ContractBatchTransfer::Erc1155 {
                contract_address: details.contract_address,
                from_address: details.from_address,
                to_address: details.to_address,
                token_ids: details.token_ids,
                hex_amounts: details.hex_amounts,
                additional_data: details.additional_data,
            }),

            _ => Err(JsValue::from_str("Unsupported contract type")),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
/// Token ID and amount of hex value pair used for ERC-1155 function
/// safeBatchTransferFrom which needs the same length of both Token ID and
/// amount arrays.
pub struct TokenAmount {
    token_id: String,
    hex_amount: String,
}

#[wasm_bindgen]
impl TokenAmount {
    /// Create an instance and serialize it to JsValue.
    #[wasm_bindgen(constructor)]
    pub fn new(token_id: String, hex_amount: String) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&Self {
            token_id,
            hex_amount,
        })
        .map_err(|e| JsValue::from_str(&format!("error: {e}")))
    }
}

impl TryFrom<JsValue> for TokenAmount {
    type Error = JsValue;

    fn try_from(val: JsValue) -> Result<Self, Self::Error> {
        val.into_serde()
            .map_err(|e| JsValue::from_str(&format!("error: {e}")))
    }
}

/// construct, sign and broadcast an approval of an ERC20/ERC721/ERC1155 token
#[wasm_bindgen]
pub async fn broadcast_approval_contract(
    details: ContractApprovalDetails,
    web3_api_url: String,
    chain_id: u64,
    private_key: PrivateKey,
) -> Result<JsValue, JsValue> {
    let receipt = broadcast_contract_approval_tx(
        details.try_into()?,
        EthNetwork::Custom {
            chain_id,
            legacy: false,
        },
        private_key.key,
        &web3_api_url,
    )
    .await
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_serde(&receipt).map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}

/// construct, sign and broadcast a transfer of an ERC20/ERC721/ERC1155 token
#[wasm_bindgen]
pub async fn broadcast_transfer_contract(
    details: ContractTransferDetails,
    web3_api_url: String,
    chain_id: u64,
    private_key: PrivateKey,
) -> Result<JsValue, JsValue> {
    let receipt = broadcast_contract_transfer_tx(
        details.try_into()?,
        EthNetwork::Custom {
            chain_id,
            legacy: false,
        },
        private_key.key,
        &web3_api_url,
    )
    .await
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_serde(&receipt).map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}

/// construct, sign and broadcast batch-transfer of an ERC1155 token
#[wasm_bindgen]
pub async fn broadcast_batch_transfer_contract(
    details: ContractBatchTransferDetails,
    web3_api_url: String,
    chain_id: u64,
    private_key: PrivateKey,
) -> Result<JsValue, JsValue> {
    let receipt = broadcast_contract_batch_transfer_tx(
        details.try_into()?,
        EthNetwork::Custom {
            chain_id,
            legacy: false,
        },
        private_key.key,
        &web3_api_url,
    )
    .await
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_serde(&receipt).map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}
