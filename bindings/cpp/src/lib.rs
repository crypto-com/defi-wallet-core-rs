use anyhow::{anyhow, Result};
use defi_wallet_core_common::{
    broadcast_tx_sync_blocking, build_signed_msg_tx, build_signed_single_msg_tx,
    get_account_balance_blocking, get_account_details_blocking, get_single_msg_sign_payload,
    BalanceApiVersion, CosmosSDKMsg, CosmosSDKTxInfo, HDWallet, Network, PublicKeyBytesWrapper,
    RawRpcAccountResponse, SecretKey, SingleCoin, WalletCoin, COMPRESSED_SECP256K1_PUBKEY_SIZE,
};
use std::sync::Arc;

/// Wrapper of `CosmosSDKMsg`
pub enum CosmosSDKMsgRaw {
    /// MsgSend
    BankSend {
        /// recipient address in bech32
        recipient_address: String,
        /// amount to send
        amount: u64,
        denom: String,
    },
    /// MsgIssueDenom
    NftIssueDenom {
        /// The denomination ID of the NFT, necessary as multiple denominations are able to be represented on each chain
        id: String,
        /// The denomination name of the NFT, necessary as multiple denominations are able to be represented on each chain.
        name: String,
        /// The account address of the user creating the denomination.
        schema: String,
    },
    /// MsgMintNft
    NftMint {
        /// The unique ID of the NFT being minted
        id: String,
        /// The unique ID of the denomination.
        denom_id: String,
        /// The name of the NFT being minted.
        name: String,
        /// The URI pointing to a JSON object that contains subsequent tokenData information off-chain
        uri: String,
        /// The data of the NFT.
        data: String,
        /// The recipient of the new NFT
        recipient: String,
    },
    /// MsgEditNft
    NftEdit {
        /// The unique ID of the NFT being edited.
        id: String,
        /// The unique ID of the denomination, necessary as multiple denominations are able to be represented on each chain.
        denom_id: String,
        /// The name of the NFT being edited.
        name: String,
        /// The URI pointing to a JSON object that contains subsequent tokenData information off-chain
        uri: String,
        /// The data of the NFT
        data: String,
    },
    /// MsgTransferNft
    NftTransfer {
        /// The unique ID of the NFT being transferred.
        id: String,
        /// The unique ID of the denomination, necessary as multiple denominations are able to be represented on each chain.
        denom_id: String,
        /// The account address who will receive the NFT as a result of the transfer transaction.
        recipient: String,
    },
    /// MsgBurnNft
    NftBurn {
        /// The ID of the Token.
        id: String,
        /// The Denom ID of the Token.
        denom_id: String,
    },
    /// MsgBeginRedelegate
    StakingBeginRedelegate {
        /// source validator address in bech32
        validator_src_address: String,
        /// destination validator address in bech32
        validator_dst_address: String,
        /// amount to redelegate
        amount: u64,
        denom: String,
    },
    /// MsgDelegate
    StakingDelegate {
        /// validator address in bech32
        validator_address: String,
        /// amount to delegate
        amount: u64,
        denom: String,
    },
    /// MsgUndelegate
    StakingUndelegate {
        /// validator address in bech32
        validator_address: String,
        /// amount to undelegate
        amount: u64,
        denom: String,
    },
    /// MsgSetWithdrawAddress
    DistributionSetWithdrawAddress {
        /// withdraw address in bech32
        withdraw_address: String,
    },
    /// MsgWithdrawDelegatorReward
    DistributionWithdrawDelegatorReward {
        /// validator address in bech32
        validator_address: String,
    },
}

impl From<&CosmosSDKMsgRaw> for CosmosSDKMsg {
    fn from(msg: &CosmosSDKMsgRaw) -> CosmosSDKMsg {
        match msg {
            CosmosSDKMsgRaw::BankSend {
                recipient_address,
                amount,
                denom,
            } => CosmosSDKMsg::BankSend {
                recipient_address: recipient_address.to_owned(),
                amount: SingleCoin::Other {
                    amount: format!("{}", amount),
                    denom: denom.to_owned(),
                },
            },
            CosmosSDKMsgRaw::NftIssueDenom { id, name, schema } => CosmosSDKMsg::NftIssueDenom {
                id: id.to_owned(),
                name: name.to_owned(),
                schema: schema.to_owned(),
            },
            CosmosSDKMsgRaw::NftMint {
                id,
                denom_id,
                name,
                uri,
                data,
                recipient,
            } => CosmosSDKMsg::NftMint {
                id: id.to_owned(),
                denom_id: denom_id.to_owned(),
                name: name.to_owned(),
                uri: uri.to_owned(),
                data: data.to_owned(),
                recipient: recipient.to_owned(),
            },
            CosmosSDKMsgRaw::NftEdit {
                id,
                denom_id,
                name,
                uri,
                data,
            } => CosmosSDKMsg::NftEdit {
                id: id.to_owned(),
                denom_id: denom_id.to_owned(),
                name: name.to_owned(),
                uri: uri.to_owned(),
                data: data.to_owned(),
            },
            CosmosSDKMsgRaw::NftTransfer {
                id,
                denom_id,
                recipient,
            } => CosmosSDKMsg::NftTransfer {
                id: id.to_owned(),
                denom_id: denom_id.to_owned(),
                recipient: recipient.to_owned(),
            },
            CosmosSDKMsgRaw::NftBurn { id, denom_id } => CosmosSDKMsg::NftBurn {
                id: id.to_owned(),
                denom_id: denom_id.to_owned(),
            },
            CosmosSDKMsgRaw::StakingDelegate {
                validator_address,
                amount,
                denom,
            } => CosmosSDKMsg::StakingDelegate {
                validator_address: validator_address.to_owned(),
                amount: SingleCoin::Other {
                    amount: format!("{}", amount),
                    denom: denom.to_owned(),
                },
            },
            CosmosSDKMsgRaw::StakingUndelegate {
                validator_address,
                amount,
                denom,
            } => CosmosSDKMsg::StakingUndelegate {
                validator_address: validator_address.to_owned(),
                amount: SingleCoin::Other {
                    amount: format!("{}", amount),
                    denom: denom.to_owned(),
                },
            },
            CosmosSDKMsgRaw::StakingBeginRedelegate {
                validator_src_address,
                validator_dst_address,
                amount,
                denom,
            } => CosmosSDKMsg::StakingBeginRedelegate {
                validator_src_address: validator_src_address.to_owned(),
                validator_dst_address: validator_dst_address.to_owned(),
                amount: SingleCoin::Other {
                    amount: format!("{}", amount),
                    denom: denom.to_owned(),
                },
            },
            CosmosSDKMsgRaw::DistributionSetWithdrawAddress { withdraw_address } => {
                CosmosSDKMsg::DistributionSetWithdrawAddress {
                    withdraw_address: withdraw_address.to_owned(),
                }
            }
            CosmosSDKMsgRaw::DistributionWithdrawDelegatorReward { validator_address } => {
                CosmosSDKMsg::DistributionWithdrawDelegatorReward {
                    validator_address: validator_address.to_owned(),
                }
            }
        }
    }
}

#[cxx::bridge(namespace = "org::defi_wallet_core")]
#[allow(clippy::too_many_arguments)]
mod ffi {

    pub enum CoinType {
        /// Crypto.org Chain mainnet
        CryptoOrgMainnet,
        /// Crypto.org Chain testnet
        CryptoOrgTestnet,
        /// Cronos mainnet beta
        CronosMainnet,
        /// Cosmos Hub mainnet
        CosmosHub,
    }

    pub enum MnemonicWordCount {
        /// Word 12
        Twelve,
        /// Word 18
        Eighteen,
        /// Word 24
        TwentyFour,
    }

    pub struct StringTuple {
        pub value: String,
        pub error: String,
    }

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
        pub memo_note: String,
        /// the network chain id
        pub chain_id: String,
        /// bech32 human readable prefix
        pub bech32hrp: String,
        /// the coin type to use
        pub coin_type: u32,
    }

    pub struct CosmosAccountInfoRaw {
        pub account_number: u64,
        pub sequence_number: u64,
    }

    extern "Rust" {
        pub fn query_account_details(api_url: String, address: String) -> Result<String>;
        pub fn query_account_details_info(
            api_url: String,
            address: String,
        ) -> Result<CosmosAccountInfoRaw>;
        pub fn broadcast_tx(tendermint_rpc_url: String, raw_signed_tx: Vec<u8>) -> Result<String>;
        pub fn query_account_balance(
            api_url: String,
            address: String,
            denom: String,
            api_version: u8,
        ) -> Result<String>;
        type PrivateKey;
        type CosmosSDKMsgRaw;
        pub fn get_msg_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            msg: &CosmosSDKMsgRaw,
        ) -> Result<Vec<u8>>;
        pub fn get_single_bank_send_signdoc(
            tx_info: CosmosSDKTxInfoRaw,
            sender_pubkey: Vec<u8>,
            recipient_address: String,
            amount: u64,
            denom: String,
        ) -> Result<Vec<u8>>;

        fn get_single_bank_send_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            recipient_address: String,
            amount: u64,
            denom: String,
        ) -> Result<Vec<u8>>;
        type Wallet;
        fn new_wallet(password: String, word_count: MnemonicWordCount) -> Result<Box<Wallet>>;

        fn restore_wallet(mnemonic: String, password: String) -> Result<Box<Wallet>>;
        fn get_default_address(self: &Wallet, coin: CoinType) -> Result<String>;
        fn get_key(self: &Wallet, derivation_path: String) -> Result<Box<PrivateKey>>;
        fn new_privatekey() -> Box<PrivateKey>;
        fn new_privatekey_from_bytes(bytes: Vec<u8>) -> Result<Box<PrivateKey>>;
        fn new_privatekey_from_hex(hex: String) -> Result<Box<PrivateKey>>;
        fn get_nft_issue_denom_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            name: String,
            schema: String,
        ) -> Result<Vec<u8>>;
        fn get_nft_mint_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
            name: String,
            uri: String,
            data: String,
            recipient: String,
        ) -> Result<Vec<u8>>;
        fn get_nft_edit_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
            name: String,
            uri: String,
            data: String,
        ) -> Result<Vec<u8>>;
        fn get_nft_transfer_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
            recipient: String,
        ) -> Result<Vec<u8>>;
        fn get_nft_burn_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
        ) -> Result<Vec<u8>>;
        fn get_staking_delegate_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_address: String,
            amount: u64,
            denom: String,
            with_reward_withdrawal: bool,
        ) -> Result<Vec<u8>>;
        fn get_staking_redelegate_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_src_address: String,
            validator_dst_address: String,
            amount: u64,
            denom: String,
            with_reward_withdrawal: bool,
        ) -> Result<Vec<u8>>;
        fn get_staking_unbond_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_address: String,
            amount: u64,
            denom: String,
            with_reward_withdrawal: bool,
        ) -> Result<Vec<u8>>;
        fn get_distribution_set_withdraw_address_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            withdraw_address: String,
        ) -> Result<Vec<u8>>;
        fn get_distribution_withdraw_reward_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_address: String,
        ) -> Result<Vec<u8>>;
    }
}

use ffi::CoinType;
impl From<CoinType> for WalletCoin {
    fn from(coin: CoinType) -> Self {
        WalletCoin::CosmosSDK {
            network: match coin {
                CoinType::CryptoOrgMainnet => Network::CryptoOrgMainnet,
                CoinType::CryptoOrgTestnet => Network::CryptoOrgTestnet,
                CoinType::CronosMainnet => Network::CronosMainnet,
                CoinType::CosmosHub => Network::CosmosHub,
                _ => Network::CryptoOrgTestnet,
            },
        }
    }
}

use ffi::MnemonicWordCount;
impl From<MnemonicWordCount> for defi_wallet_core_common::MnemonicWordCount {
    fn from(word_count: MnemonicWordCount) -> Self {
        match word_count {
            MnemonicWordCount::Twelve => defi_wallet_core_common::MnemonicWordCount::Twelve,
            MnemonicWordCount::Eighteen => defi_wallet_core_common::MnemonicWordCount::Eighteen,
            _ => defi_wallet_core_common::MnemonicWordCount::TwentyFour,
        }
    }
}

pub struct PrivateKey {
    key: Arc<SecretKey>,
}

/// generates a random private key
fn new_privatekey() -> Box<PrivateKey> {
    Box::new(PrivateKey {
        key: Arc::new(SecretKey::new()),
    })
}

/// constructs private key from bytes
fn new_privatekey_from_bytes(bytes: Vec<u8>) -> Result<Box<PrivateKey>> {
    Ok(Box::new(PrivateKey {
        key: Arc::new(SecretKey::from_bytes(bytes)?),
    }))
}

/// constructs private key from hex
fn new_privatekey_from_hex(hex: String) -> Result<Box<PrivateKey>> {
    Ok(Box::new(PrivateKey {
        key: Arc::new(SecretKey::from_hex(hex)?),
    }))
}

impl PrivateKey {
    /// gets public key to byte array
    pub fn get_public_key_bytes(&self) -> Vec<u8> {
        self.key.get_public_key_bytes()
    }

    /// gets public key to a hex string without the 0x prefix
    pub fn get_public_key_hex(&self) -> String {
        self.key.get_public_key_hex()
    }

    /// converts private key to byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes()
    }

    /// converts private key to a hex string without the 0x prefix
    pub fn to_hex(&self) -> String {
        self.key.to_hex()
    }
}

pub struct Wallet {
    wallet: HDWallet,
}

fn new_wallet(password: String, word_count: MnemonicWordCount) -> Result<Box<Wallet>> {
    let wallet = HDWallet::generate_wallet(Some(password), Some(word_count.into()))?;
    Ok(Box::new(Wallet { wallet }))
}

fn restore_wallet(mnemonic: String, password: String) -> Result<Box<Wallet>> {
    let wallet = HDWallet::recover_wallet(mnemonic, Some(password))?;
    Ok(Box::new(Wallet { wallet }))
}

impl Wallet {
    pub fn get_default_address(&self, coin: CoinType) -> Result<String> {
        Ok(self.wallet.get_default_address(coin.into())?)
    }

    pub fn get_key(&self, derivation_path: String) -> Result<Box<PrivateKey>> {
        let key = self.wallet.get_key(derivation_path)?;
        Ok(Box::new(PrivateKey { key }))
    }
}

impl From<ffi::CosmosSDKTxInfoRaw> for CosmosSDKTxInfo {
    fn from(info: ffi::CosmosSDKTxInfoRaw) -> Self {
        CosmosSDKTxInfo {
            account_number: info.account_number,
            sequence_number: info.sequence_number,
            gas_limit: info.gas_limit,
            fee_amount: SingleCoin::Other {
                amount: info.fee_amount.to_string(),
                denom: info.fee_denom,
            },
            timeout_height: info.timeout_height,
            memo_note: Some(info.memo_note),
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
pub fn get_single_bank_send_signdoc(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    sender_pubkey: Vec<u8>,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>> {
    if sender_pubkey.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
        return Err(anyhow!(
            "invalid sender pubkey length: {}",
            sender_pubkey.len()
        ));
    }
    let pubkey = PublicKeyBytesWrapper(sender_pubkey);
    let signed_document = get_single_msg_sign_payload(
        tx_info.into(),
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
        pubkey,
    )?;
    Ok(signed_document.to_vec())
}

/// creates the signed transaction
/// for `MsgSend` from the Cosmos SDK bank module
pub fn get_single_bank_send_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    recipient_address: String,
    amount: u64,
    denom: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::BankSend {
            recipient_address,
            amount: SingleCoin::Other {
                amount: format!("{}", amount),
                denom,
            },
        },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgIssueDenom` from the Chainmain nft module
fn get_nft_issue_denom_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    name: String,
    schema: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::NftIssueDenom { id, name, schema },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgMintNft` from the Chainmain nft module
#[allow(clippy::too_many_arguments)]
fn get_nft_mint_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    recipient: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::NftMint {
            id,
            denom_id,
            name,
            uri,
            data,
            recipient,
        },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgEditNft` from the Chainmain nft module
fn get_nft_edit_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::NftEdit {
            id,
            denom_id,
            name,
            uri,
            data,
        },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgTransferNft` from the Chainmain nft module
fn get_nft_transfer_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
    recipient: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::NftTransfer {
            id,
            denom_id,
            recipient,
        },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgBurnNft` from the Chainmain nft module
fn get_nft_burn_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::NftBurn { id, denom_id },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgDelegate` from the Cosmos SDK staking module
pub fn get_staking_delegate_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    validator_address: String,
    amount: u64,
    denom: String,
    with_reward_withdrawal: bool,
) -> Result<Vec<u8>> {
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

    build_signed_msg_tx(tx_info.into(), messages, private_key.key.clone()).map_err(|e| e.into())
}

/// creates the signed transaction
/// for `MsgBeginRedelegate` from the Cosmos SDK staking module
pub fn get_staking_redelegate_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    validator_src_address: String,
    validator_dst_address: String,
    amount: u64,
    denom: String,
    with_reward_withdrawal: bool,
) -> Result<Vec<u8>> {
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

    build_signed_msg_tx(tx_info.into(), messages, private_key.key.clone()).map_err(|e| e.into())
}

/// creates the signed transaction
/// for `MsgUndelegate` from the Cosmos SDK staking module
pub fn get_staking_unbond_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    validator_address: String,
    amount: u64,
    denom: String,
    with_reward_withdrawal: bool,
) -> Result<Vec<u8>> {
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

    build_signed_msg_tx(tx_info.into(), messages, private_key.key.clone()).map_err(|e| e.into())
}

/// creates the signed transaction
/// for `MsgSetWithdrawAddress` from the Cosmos SDK distributon module
pub fn get_distribution_set_withdraw_address_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    withdraw_address: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::DistributionSetWithdrawAddress { withdraw_address },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgWithdrawDelegatorReward` from the Cosmos SDK distributon module
pub fn get_distribution_withdraw_reward_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    validator_address: String,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(
        tx_info.into(),
        CosmosSDKMsg::DistributionWithdrawDelegatorReward { validator_address },
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction
pub fn get_msg_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    msg: &CosmosSDKMsgRaw,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(tx_info.into(), msg.into(), private_key.key.clone())?;
    Ok(ret)
}
pub fn query_account_details(api_url: String, address: String) -> Result<String> {
    let account_details: RawRpcAccountResponse = get_account_details_blocking(&api_url, &address)?;
    Ok(serde_json::to_string(&account_details)?)
}

pub fn query_account_details_info(
    api_url: String,
    address: String,
) -> Result<ffi::CosmosAccountInfoRaw> {
    let account_details: RawRpcAccountResponse = get_account_details_blocking(&api_url, &address)?;

    match account_details {
        RawRpcAccountResponse::OkResponse { account } => Ok(ffi::CosmosAccountInfoRaw {
            account_number: account.account_number,
            sequence_number: account.sequence,
        }),
        RawRpcAccountResponse::ErrorResponse {
            code,
            message,
            details,
        } => Err(anyhow!(
            "RawRpcAccountResponse error {} {} {:?}",
            code,
            message,
            details
        )),
    }
}

pub fn query_account_balance(
    api_url: String,
    address: String,
    denom: String,
    api_version: u8,
) -> Result<String> {
    let balance_api_version = BalanceApiVersion::from(api_version);

    let account_details =
        get_account_balance_blocking(&api_url, &address, &denom, balance_api_version)?;

    Ok(serde_json::to_string(&account_details)?)
}

pub fn broadcast_tx(tendermint_rpc_url: String, raw_signed_tx: Vec<u8>) -> Result<String> {
    let resp = broadcast_tx_sync_blocking(&tendermint_rpc_url, raw_signed_tx)?;
    Ok(serde_json::to_string(&resp)?)
}
