#![allow(clippy::unused_unit)]
/// TODO: remove when a new version is published https://github.com/crypto-com/defi-wallet-core-rs/issues/110
use std::sync::Arc;

use defi_wallet_core_common::{
    broadcast_contract_approval_tx, broadcast_contract_transfer_tx, broadcast_sign_eth_tx,
    broadcast_tx_sync, build_signed_msg_tx, build_signed_single_msg_tx, get_account_balance,
    get_account_details, get_contract_balance, get_eth_balance, get_single_msg_sign_payload,
    BalanceApiVersion, ContractApproval, ContractBalance, ContractTransfer, CosmosSDKMsg,
    CosmosSDKTxInfo, EthAmount, EthNetwork, HDWallet, Height, Network, PublicKeyBytesWrapper,
    SecretKey, SingleCoin, WalletCoin, COMPRESSED_SECP256K1_PUBKEY_SIZE,
};

use defi_wallet_core_common::node;
use defi_wallet_core_common::transaction;

use wasm_bindgen::prelude::*;
/// wasm utilities
mod utils;

/// HD Wallet wrapper for Wasm
#[wasm_bindgen]
pub struct Wallet {
    wallet: HDWallet,
}

/// Signing key wrapper for Wasm
#[derive(Clone)]
#[wasm_bindgen]
pub struct PrivateKey {
    key: Arc<SecretKey>,
}

#[wasm_bindgen]
impl PrivateKey {
    /// generates a random private key
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            key: Arc::new(SecretKey::new()),
        }
    }

    /// constructs private key from bytes
    #[wasm_bindgen]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<PrivateKey, JsValue> {
        Ok(Self {
            key: Arc::new(
                SecretKey::from_bytes(bytes)
                    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?,
            ),
        })
    }

    /// constructs private key from hex
    #[wasm_bindgen]
    pub fn from_hex(hex: String) -> Result<PrivateKey, JsValue> {
        Ok(Self {
            key: Arc::new(
                SecretKey::from_hex(hex)
                    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?,
            ),
        })
    }

    /// gets public key to byte array
    #[wasm_bindgen]
    pub fn get_public_key_bytes(&self) -> Vec<u8> {
        self.key.get_public_key_bytes()
    }

    /// gets public key to a hex string without the 0x prefix
    #[wasm_bindgen]
    pub fn get_public_key_hex(&self) -> String {
        self.key.get_public_key_hex()
    }

    /// converts private key to byte array
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes()
    }

    /// converts private key to a hex string without the 0x prefix
    #[wasm_bindgen]
    pub fn to_hex(&self) -> String {
        self.key.to_hex()
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        Self::new()
    }
}

/// basic supported coins for wasm
/// TODO: re-work with `Network`
/// (wasm only supports C-style enums)
#[wasm_bindgen]
pub enum CoinType {
    /// Crypto.org Chain mainnet
    CryptoOrgMainnet,
    /// Crypto.org Chain testnet
    CryptoOrgTestnet,
    /// Cronos mainnet beta
    CronosMainnet,
    /// Cosmos Hub mainnet
    CosmosHub,
    /// Ethereum
    Ethereum,
}

impl From<CoinType> for WalletCoin {
    fn from(coin: CoinType) -> Self {
        match coin {
            CoinType::CryptoOrgMainnet => WalletCoin::CosmosSDK {
                network: Network::CryptoOrgMainnet,
            },
            CoinType::CryptoOrgTestnet => WalletCoin::CosmosSDK {
                network: Network::CryptoOrgTestnet,
            },
            CoinType::CronosMainnet => WalletCoin::CosmosSDK {
                network: Network::CronosMainnet,
            },
            CoinType::CosmosHub => WalletCoin::CosmosSDK {
                network: Network::CosmosHub,
            },
            CoinType::Ethereum => WalletCoin::Ethereum,
        }
    }
}

#[wasm_bindgen]
pub enum MnemonicWordCount {
    /// Word 12
    Twelve,
    /// Word 18
    Eighteen,
    /// Word 24
    TwentyFour,
}

impl From<MnemonicWordCount> for defi_wallet_core_common::MnemonicWordCount {
    fn from(word_count: MnemonicWordCount) -> Self {
        match word_count {
            MnemonicWordCount::Twelve => defi_wallet_core_common::MnemonicWordCount::Twelve,
            MnemonicWordCount::Eighteen => defi_wallet_core_common::MnemonicWordCount::Eighteen,
            MnemonicWordCount::TwentyFour => defi_wallet_core_common::MnemonicWordCount::TwentyFour,
        }
    }
}

#[wasm_bindgen]
impl Wallet {
    /// generate a random wallet (with an optional password)
    #[wasm_bindgen(constructor)]
    pub fn new(
        password: Option<String>,
        word_count: Option<MnemonicWordCount>,
    ) -> Result<Wallet, JsValue> {
        let wallet = HDWallet::generate_wallet(password, word_count.map(|val| val.into()))
            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;
        Ok(Self { wallet })
    }

    /// recovers/imports HD wallet from a BIP39 backup phrase (English words) and an optional password
    #[wasm_bindgen]
    pub fn recover_wallet(
        mnemonic_phase: String,
        password: Option<String>,
    ) -> Result<Wallet, JsValue> {
        let wallet = HDWallet::recover_wallet(mnemonic_phase, password)
            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;
        Ok(Self { wallet })
    }

    /// return the default address for a given coin type
    #[wasm_bindgen]
    pub fn get_default_address(&self, coin: CoinType) -> Result<String, JsValue> {
        self.wallet
            .get_default_address(coin.into())
            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    /// obtain a signing key for a given derivation path
    #[wasm_bindgen]
    pub fn get_key(&self, derivation_path: String) -> Result<PrivateKey, JsValue> {
        let key = self
            .wallet
            .get_key(derivation_path)
            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;
        Ok(PrivateKey { key })
    }
}

/// the common transaction data needed for Cosmos SDK transactions
/// (raw duplicate needed for Wasm -- TODO: unify common structures?)
#[wasm_bindgen(getter_with_clone)]
pub struct CosmosSDKTxInfoRaw {
    /// global account number of the sender
    pub account_number: u64,
    /// equivalent of "account nonce"
    pub sequence_number: u64,
    /// the maximum gas limit
    pub gas_limit: u64,
    /// the amount fee to be paid (gas_limit * gas_price)
    pub fee_amount: u64,
    /// the fee's denomination
    pub fee_denom: String,
    /// transaction timeout
    pub timeout_height: u32,
    /// optional memo
    pub memo_note: Option<String>,
    /// the network chain id
    pub chain_id: String,
    /// bech32 human readable prefix
    pub bech32hrp: String,
    /// the coin type to use
    pub coin_type: u32,
}

#[wasm_bindgen]
impl CosmosSDKTxInfoRaw {
    /// constructor for JS -- TODO: some builder API wrapper?
    #[wasm_bindgen(constructor)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_number: u64,
        sequence_number: u64,
        gas_limit: u64,
        fee_amount: u64,
        fee_denom: String,
        timeout_height: u32,
        memo_note: Option<String>,
        chain_id: String,
        bech32hrp: String,
        coin_type: u32,
    ) -> Self {
        Self {
            account_number,
            sequence_number,
            gas_limit,
            fee_amount,
            fee_denom,
            timeout_height,
            memo_note,
            chain_id,
            bech32hrp,
            coin_type,
        }
    }
}

impl From<CosmosSDKTxInfoRaw> for CosmosSDKTxInfo {
    fn from(info: CosmosSDKTxInfoRaw) -> Self {
        CosmosSDKTxInfo {
            account_number: info.account_number,
            sequence_number: info.sequence_number,
            gas_limit: info.gas_limit,
            fee_amount: SingleCoin::Other {
                amount: info.fee_amount.to_string(),
                denom: info.fee_denom,
            },
            timeout_height: info.timeout_height,
            memo_note: info.memo_note,
            network: Network::Other {
                chain_id: info.chain_id,
                coin_type: info.coin_type,
                bech32hrp: info.bech32hrp,
            },
        }
    }
}

/// creates the transaction signing payload (`SignDoc`)
/// for `MsgSend` from the Cosmos SDK bank module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_single_bank_send_signdoc(
    tx_info: CosmosSDKTxInfoRaw,
    sender_pubkey: Vec<u8>,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>, JsValue> {
    if sender_pubkey.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
        return Err(JsValue::from_str("invalid public key length"));
    }
    let pubkey = PublicKeyBytesWrapper(sender_pubkey);
    get_single_msg_sign_payload(
        tx_info.into(),
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
        pubkey,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for MsgSend from the Cosmos SDK bank module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_single_bank_send_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>, JsValue> {
    build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
        private_key.key,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgIssueDenom` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_issue_denom_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    name: String,
    schema: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_issue_denom_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        name,
        schema,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgMintNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn get_nft_mint_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    recipient: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_mint_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        denom_id,
        name,
        uri,
        data,
        recipient,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgEditNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_edit_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_edit_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        denom_id,
        name,
        uri,
        data,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgTransferNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_transfer_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
    recipient: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_transfer_signed_tx(
        tx_info.into(),
        private_key.key,
        id,
        denom_id,
        recipient,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `MsgBurnNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_nft_burn_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    id: String,
    denom_id: String,
) -> Result<Vec<u8>, JsValue> {
    transaction::nft::get_nft_burn_signed_tx(tx_info.into(), private_key.key, id, denom_id)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `StakingDelegate` from the Chainmain staking module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_staking_delegate_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    validator_address: String,
    amount: u64,
    denom: String,
    with_reward_withdrawal: bool,
) -> Result<Vec<u8>, JsValue> {
    let mut messages = vec![CosmosSDKMsg::StakingDelegate {
        validator_address: validator_address.clone(),
        amount: SingleCoin::Other {
            amount: format!("{}", amount),
            denom,
        },
    }];

    if with_reward_withdrawal {
        messages.push(CosmosSDKMsg::DistributionWithdrawDelegatorReward { validator_address });
    }

    build_signed_msg_tx(tx_info.into(), messages, private_key.key)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `StakingBeginRedelegate` from the Chainmain staking module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_staking_redelegate_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    validator_src_address: String,
    validator_dst_address: String,
    amount: u64,
    denom: String,
    with_reward_withdrawal: bool,
) -> Result<Vec<u8>, JsValue> {
    let mut messages = vec![CosmosSDKMsg::StakingBeginRedelegate {
        validator_src_address: validator_src_address.clone(),
        validator_dst_address: validator_dst_address.clone(),
        amount: SingleCoin::Other {
            amount: format!("{}", amount),
            denom,
        },
    }];

    if with_reward_withdrawal {
        messages.push(CosmosSDKMsg::DistributionWithdrawDelegatorReward {
            validator_address: validator_src_address,
        });
        messages.push(CosmosSDKMsg::DistributionWithdrawDelegatorReward {
            validator_address: validator_dst_address,
        });
    }

    build_signed_msg_tx(tx_info.into(), messages, private_key.key)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `StakingUndelegate` from the Chainmain staking module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_staking_unbond_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    validator_address: String,
    amount: u64,
    denom: String,
    with_reward_withdrawal: bool,
) -> Result<Vec<u8>, JsValue> {
    let mut messages = vec![CosmosSDKMsg::StakingUndelegate {
        validator_address: validator_address.clone(),
        amount: SingleCoin::Other {
            amount: format!("{}", amount),
            denom,
        },
    }];

    if with_reward_withdrawal {
        messages.push(CosmosSDKMsg::DistributionWithdrawDelegatorReward { validator_address });
    }

    build_signed_msg_tx(tx_info.into(), messages, private_key.key)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `DistributionSetWithdrawAddress` from the Chainmain distribution module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_distribution_set_withdraw_address_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    withdraw_address: String,
) -> Result<Vec<u8>, JsValue> {
    build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::DistributionSetWithdrawAddress { withdraw_address },
        private_key.key,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `DistributionWithdrawDelegatorReward` from the Chainmain distribution module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_distribution_withdraw_reward_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    validator_address: String,
) -> Result<Vec<u8>, JsValue> {
    build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::DistributionWithdrawDelegatorReward { validator_address },
        private_key.key,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// creates the signed transaction
/// for `IbcTransfer` from the Chainmain ibc module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[wasm_bindgen]
pub fn get_ibc_transfer_signed_tx(
    tx_info: CosmosSDKTxInfoRaw,
    private_key: PrivateKey,
    receiver: String,
    source_port: String,
    source_channel: String,
    denom: String,
    token: u64,
    revision_height: u64,
    revision_number: u64,
    timeout_timestamp: u64,
) -> Result<Vec<u8>, JsValue> {
    // TODO: Need to support converting receiver from hex address to bech32 here.

    build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::IbcTransfer {
            receiver,
            source_port,
            source_channel,
            token: SingleCoin::Other {
                amount: format!("{}", token),
                denom,
            },
            timeout_height: Height {
                revision_height,
                revision_number,
            },
            timeout_timestamp,
        },
        private_key.key,
    )
    .map_err(|e| JsValue::from_str(&format!("error: {}", e)))
}

/// retrieves the account details (e.g. sequence and account number) for a given address
/// TODO: switch to grpc-web
#[wasm_bindgen]
pub async fn query_account_details(api_url: String, address: String) -> Result<JsValue, JsValue> {
    let account_details = get_account_details(&api_url, &address)
        .await
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_serde(&account_details)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}

/// retrieves the account balance for a given address and a denom
/// api-version: https://github.com/cosmos/cosmos-sdk/releases/tag/v0.42.11
/// - 0 means before 0.42.11 or 0.44.4
/// - >=1 means after 0.42.11 or 0.44.4
/// TODO: switch to grpc-web
#[wasm_bindgen]
pub async fn query_account_balance(
    api_url: String,
    address: String,
    denom: String,
    api_version: u8,
) -> Result<JsValue, JsValue> {
    let balance_api_version = if api_version == 0 {
        BalanceApiVersion::Old
    } else {
        BalanceApiVersion::New
    };
    let account_details = get_account_balance(&api_url, &address, &denom, balance_api_version)
        .await
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

    Ok(JsValue::from_serde(&account_details)
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}

/// broadcasts a signed cosmos sdk tx
#[wasm_bindgen]
pub async fn broadcast_tx(
    tendermint_rpc_url: String,
    raw_signed_tx: Vec<u8>,
) -> Result<JsValue, JsValue> {
    let resp = broadcast_tx_sync(&tendermint_rpc_url, raw_signed_tx)
        .await
        .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?
        .into_result()
        .map_err(|e| JsValue::from_str(&format!("missing_result: {}", e)))?;

    if let tendermint::abci::Code::Err(_) = resp.code {
        return Err(
            JsValue::from_serde(&resp).map_err(|e| JsValue::from_str(&format!("error: {}", e)))?
        );
    }

    Ok(JsValue::from_serde(&resp).map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
}

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

/// construct, sign and broadcast an approval of a ERC20/ERC721/ERC1155 token
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

/// construct, sign and broadcast a transfer of a ERC20/ERC721/ERC1155 token
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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Grpc Web Client wrapper for Wasm
#[wasm_bindgen]
pub struct GrpcWebClient(node::nft::Client);

impl GrpcWebClient {
    pub fn new(grpc_web_url: String) -> Self {
        Self(node::nft::Client::new(grpc_web_url))
    }
    pub async fn supply(&mut self, denom_id: String, owner: String) -> Result<JsValue, JsValue> {
        let supply = self.0.supply(denom_id, owner).await?;
        JsValue::from_serde(&supply).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn owner(&mut self, denom_id: String, owner: String) -> Result<JsValue, JsValue> {
        let owner = self.0.owner(denom_id, owner).await?;
        JsValue::from_serde(&owner).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn collection(&mut self, denom_id: String) -> Result<JsValue, JsValue> {
        let collection = self.0.collection(denom_id).await?;
        JsValue::from_serde(&collection).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn denom(&mut self, denom_id: String) -> Result<JsValue, JsValue> {
        let denom = self.0.denom(denom_id).await?;
        JsValue::from_serde(&denom).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn denom_by_name(&mut self, denom_name: String) -> Result<JsValue, JsValue> {
        let denom = self.0.denom_by_name(denom_name).await?;
        JsValue::from_serde(&denom).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn denoms(&mut self) -> Result<JsValue, JsValue> {
        let denoms = self.0.denoms().await?;
        JsValue::from_serde(&denoms).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }

    pub async fn nft(&mut self, denom_id: String, token_id: String) -> Result<JsValue, JsValue> {
        let nft = self.0.nft(denom_id, token_id).await?;
        JsValue::from_serde(&nft).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
    }
}
