use anyhow::{anyhow, Result};
use cxx::{type_id, ExternType};
use defi_wallet_core_common::node::ethereum::provider::set_ethers_httpagent;
use defi_wallet_core_common::{
    broadcast_tx_sync_blocking, build_signed_msg_tx, build_signed_single_msg_tx,
    get_account_balance_blocking, get_account_details_blocking, get_single_msg_sign_payload,
    CosmosSDKMsg, CosmosSDKTxInfo, EthError, EthNetwork, EthTxInfo, HDWallet, Height, LoginInfo,
    Network, PublicKeyBytesWrapper, RawRpcAccountResponse, SecretKey, SingleCoin,
    TransactionReceipt, TxBroadcastResult, WalletCoin, COMPRESSED_SECP256K1_PUBKEY_SIZE,
};

use ethers::types::Signature;
use std::str::FromStr;
use std::sync::Arc;
mod nft;

mod contract;

mod uint;

mod ethereum;

/// Wrapper of `CosmosSDKMsg`
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
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
    /// MsgTransfer
    IbcTransfer {
        /// the recipient address on the destination chain
        receiver: String,
        /// the port on which the packet will be sent
        source_port: String,
        /// the channel by which the packet will be sent
        source_channel: String,
        /// the tokens to be transferred
        denom: String,
        token: u64,
        /// Timeout height relative to the current block height.
        /// The timeout is disabled when set to 0.
        revision_height: u64,
        revision_number: u64,
        /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
        /// The timeout is disabled when set to 0.
        timeout_timestamp: u64,
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
            CosmosSDKMsgRaw::IbcTransfer {
                receiver,
                source_port,
                source_channel,
                denom,
                token,
                revision_height,
                revision_number,
                timeout_timestamp,
            } => CosmosSDKMsg::IbcTransfer {
                receiver: receiver.to_owned(),
                source_port: source_port.to_owned(),
                source_channel: source_channel.to_owned(),
                token: SingleCoin::Other {
                    amount: format!("{}", token),
                    denom: denom.to_owned(),
                },
                timeout_height: Height {
                    revision_height: *revision_height,
                    revision_number: *revision_number,
                },
                timeout_timestamp: *timeout_timestamp,
            },
        }
    }
}

/// wrapper for LoginInfo
pub struct CppLoginInfo {
    pub logininfo: LoginInfo,
}

#[cxx::bridge(namespace = "org::defi_wallet_core")]
#[allow(clippy::too_many_arguments)]
pub mod ffi {

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

    pub enum MnemonicWordCount {
        /// Word 12
        Twelve,
        /// Word 18
        Eighteen,
        /// Word 24
        TwentyFour,
    }

    pub enum EthAmount {
        /// 10^-18 ETH
        WeiDecimal,
        /// 10^-9 ETH
        GweiDecimal,
        EthDecimal,
    }

    pub struct EthTxInfoRaw {
        pub to_address: String,
        pub amount: String,
        pub amount_unit: EthAmount,
        pub nonce: String,
        pub gas_limit: String,
        pub gas_price: String,
        pub gas_price_unit: EthAmount,
        pub data: Vec<u8>,
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
    #[derive(Debug, Default)]
    pub struct CosmosTransactionReceiptRaw {
        /// tendermint transaction hash in hexadecimal
        pub tx_hash_hex: String,
        /// error code (0 if success)
        pub code: u32,
        /// possible error log
        pub log: String,
    }

    #[derive(Debug, Default)]
    pub struct CronosTransactionReceiptRaw {
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

    extern "C++" {
        include!("defi-wallet-core-cpp/src/uint.rs.h");
        type U256 = crate::uint::ffi::U256;
    }

    extern "Rust" {
        /// query account details from cosmos address
        pub fn query_account_details(api_url: String, address: String) -> Result<String>;
        /// query account details info from cosmos address
        pub fn query_account_details_info(
            api_url: String,
            address: String,
        ) -> Result<CosmosAccountInfoRaw>;
        /// broadcast the cosmos transaction
        pub fn broadcast_tx(
            tendermint_rpc_url: String,
            raw_signed_tx: Vec<u8>,
        ) -> Result<CosmosTransactionReceiptRaw>;
        /// query account balance from cosmos address and denom name
        pub fn query_account_balance(
            grpc_url: String,
            address: String,
            denom: String,
        ) -> Result<String>;
        type PrivateKey;
        type CosmosSDKMsgRaw;
        /// creates the signed transaction for cosmos
        pub fn get_msg_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            msg: &CosmosSDKMsgRaw,
        ) -> Result<Vec<u8>>;
        /// creates the transaction signing payload (`SignDoc`)
        /// for `MsgSend` from the Cosmos SDK bank module
        pub fn get_single_bank_send_signdoc(
            tx_info: CosmosSDKTxInfoRaw,
            sender_pubkey: Vec<u8>,
            recipient_address: String,
            amount: u64,
            denom: String,
        ) -> Result<Vec<u8>>;

        /// creates the signed transaction
        /// for `MsgSend` from the Cosmos SDK bank module
        fn get_single_bank_send_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            recipient_address: String,
            amount: u64,
            denom: String,
        ) -> Result<Vec<u8>>;
        type Wallet;
        /// generates the HD wallet with a BIP39 backup phrase (English words) and password
        fn new_wallet(password: String, word_count: MnemonicWordCount) -> Result<Box<Wallet>>;

        /// get backup mnemonic phrase
        fn get_backup_mnemonic_phrase(self: &Wallet) -> Result<String>;

        /// generate mnemonics
        fn generate_mnemonics(password: String, word_count: MnemonicWordCount) -> Result<String>;

        /// recovers/imports HD wallet from a BIP39 backup phrase (English words) and password
        fn restore_wallet(mnemonic: String, password: String) -> Result<Box<Wallet>>;
        /// returns the default address of the wallet
        fn get_default_address(self: &Wallet, coin: CoinType) -> Result<String>;
        /// returns the address from index in wallet
        fn get_address(self: &Wallet, coin: CoinType, index: u32) -> Result<String>;
        /// returns the ethereum address from index in wallet
        fn get_eth_address(self: &Wallet, index: u32) -> Result<String>;
        /// return the secret key for a given derivation path
        fn get_key(self: &Wallet, derivation_path: String) -> Result<Box<PrivateKey>>;
        /// generates a random private key
        fn new_privatekey() -> Box<PrivateKey>;
        /// constructs private key from bytes
        fn new_privatekey_from_bytes(bytes: Vec<u8>) -> Result<Box<PrivateKey>>;
        /// constructs private key from hex string
        fn new_privatekey_from_hex(hex: String) -> Result<Box<PrivateKey>>;
        /// creates the signed transaction
        /// for `MsgDelegate` from the Cosmos SDK staking module
        fn get_staking_delegate_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_address: String,
            amount: u64,
            denom: String,
            with_reward_withdrawal: bool,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgBeginRedelegate` from the Cosmos SDK staking module
        fn get_staking_redelegate_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_src_address: String,
            validator_dst_address: String,
            amount: u64,
            denom: String,
            with_reward_withdrawal: bool,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgUndelegate` from the Cosmos SDK staking module
        fn get_staking_unbond_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_address: String,
            amount: u64,
            denom: String,
            with_reward_withdrawal: bool,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgSetWithdrawAddress` from the Cosmos SDK distributon module
        fn get_distribution_set_withdraw_address_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            withdraw_address: String,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgWithdrawDelegatorReward` from the Cosmos SDK distributon module
        fn get_distribution_withdraw_reward_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            validator_address: String,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgTransfer` from the Cosmos SDK ibc module
        fn get_ibc_transfer_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            receiver: String,
            source_port: String,
            source_channel: String,
            denom: String,
            token: u64,
            revision_height: u64,
            revision_number: u64,
            timeout_timestamp: u64,
        ) -> Result<Vec<u8>>;

        type CppLoginInfo;
        /// Create Login Info by `msg`
        /// all information from the EIP-4361 plaintext message:
        /// https://eips.ethereum.org/EIPS/eip-4361
        fn new_logininfo(msg: String) -> Result<Box<CppLoginInfo>>;
        /// Sign Login Info
        /// constructs the plaintext message and signs it according to EIP-191
        /// (as per EIP-4361). The returned vector is a serialized recoverable signature
        /// (as used in Ethereum).
        fn sign_logininfo(self: &CppLoginInfo, private_key: &PrivateKey) -> Result<Vec<u8>>;
        /// Verify Login Info
        /// It verified the signature matches + also verifies the content of the message:
        /// - address in the message matches the address recovered from the signature
        /// - the time is valid
        /// ...
        /// NOTE: the server may still need to do extra verifications according to its needs
        /// (e.g. verify chain-id, nonce, uri + possibly fetch additional data associated
        /// with the given Ethereum address, such as ERC-20/ERC-721/ERC-1155 asset ownership)
        fn verify_logininfo(self: &CppLoginInfo, signature: &[u8]) -> Result<()>;

        /// create cronos tx info to sign
        pub fn new_eth_tx_info() -> EthTxInfoRaw;

        /// sign cronos tx with private key
        pub fn build_eth_signed_tx(
            tx_info: EthTxInfoRaw,
            network: &str,
            secret_key: &PrivateKey,
        ) -> Result<Vec<u8>>;
        #[cxx_name = "build_eth_signed_tx"]
        /// sign cronos tx with private key in custom network
        pub fn build_custom_eth_signed_tx(
            tx_info: EthTxInfoRaw,
            chain_id: u64,
            legacy: bool,
            secret_key: &PrivateKey,
        ) -> Result<Vec<u8>>;

        /// given the account address, it returns the amount of native token it owns
        pub fn get_eth_balance(address: &str, api_url: &str) -> Result<U256>;

        /// Returns the corresponding account's nonce / number of transactions
        /// sent from it.
        pub fn get_eth_nonce(address: &str, api_url: &str) -> Result<String>;

        /// broadcast signed cronos tx
        pub fn broadcast_eth_signed_raw_tx(
            raw_tx: Vec<u8>,
            web3api_url: &str,
            polling_interval_ms: u64,
        ) -> Result<CronosTransactionReceiptRaw>;

        /// set cronos http-agent name
        pub fn set_cronos_httpagent(agent: &str) -> Result<()>;

    } // end of RUST block
} // end of ffi block

use ffi::CronosTransactionReceiptRaw;
impl From<TransactionReceipt> for CronosTransactionReceiptRaw {
    fn from(src: TransactionReceipt) -> Self {
        ffi::CronosTransactionReceiptRaw {
            transaction_hash: src.transaction_hash,
            transaction_index: src.transaction_index,
            block_hash: src.block_hash,
            block_number: src.block_number,
            cumulative_gas_used: src.cumulative_gas_used,
            gas_used: src.gas_used,
            contract_address: src.contract_address,
            status: src.status,
            root: src.root,
            logs_bloom: src.logs_bloom,
            transaction_type: src.transaction_type,
            effective_gas_price: src.effective_gas_price,
            logs: src.logs,
        }
    }
}

impl From<TxBroadcastResult> for ffi::CosmosTransactionReceiptRaw {
    fn from(src: TxBroadcastResult) -> Self {
        ffi::CosmosTransactionReceiptRaw {
            tx_hash_hex: src.tx_hash_hex,
            code: src.code,
            log: src.log,
        }
    }
}

use ffi::CoinType;
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
            _ => WalletCoin::Ethereum {
                network: EthNetwork::Mainnet,
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
unsafe impl ExternType for PrivateKey {
    type Id = type_id!("org::defi_wallet_core::PrivateKey");
    type Kind = cxx::kind::Opaque;
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

/// constructs private key from hex string
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

/// generates the HD wallet with a BIP39 backup phrase (English words) and password
fn new_wallet(password: String, word_count: MnemonicWordCount) -> Result<Box<Wallet>> {
    let wallet = HDWallet::generate_wallet(Some(password), Some(word_count.into()))?;
    Ok(Box::new(Wallet { wallet }))
}

/// generate mnemonics
fn generate_mnemonics(password: String, word_count: MnemonicWordCount) -> Result<String> {
    let wallet = HDWallet::generate_wallet(Some(password), Some(word_count.into()))?;
    wallet
        .get_backup_mnemonic_phrase()
        .ok_or_else(|| anyhow!("Cannot generate new mnemonics"))
}

/// recovers/imports HD wallet from a BIP39 backup phrase (English words) and password
fn restore_wallet(mnemonic: String, password: String) -> Result<Box<Wallet>> {
    let wallet = HDWallet::recover_wallet(mnemonic, Some(password))?;
    Ok(Box::new(Wallet { wallet }))
}

impl Wallet {
    /// get backup mnemonic phrase
    fn get_backup_mnemonic_phrase(self: &Wallet) -> Result<String> {
        self.wallet
            .get_backup_mnemonic_phrase()
            .ok_or_else(|| anyhow!("No backup mnemonic phrase"))
    }

    /// returns the default address of the wallet
    pub fn get_default_address(&self, coin: CoinType) -> Result<String> {
        self.get_address(coin, 0)
    }

    /// returns the address from index in wallet
    pub fn get_address(&self, coin: CoinType, index: u32) -> Result<String> {
        Ok(self.wallet.get_address(coin.into(), index)?)
    }

    /// returns the ethereum address from index in wallet
    pub fn get_eth_address(&self, index: u32) -> Result<String> {
        self.get_address(CoinType::Ethereum, index)
    }

    /// return the secret key for a given derivation path
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
/// for `MsgTransfer` from the Cosmos SDK ibc module
#[allow(clippy::too_many_arguments)]
pub fn get_ibc_transfer_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    receiver: String,
    source_port: String,
    source_channel: String,
    denom: String,
    token: u64,
    revision_height: u64,
    revision_number: u64,
    timeout_timestamp: u64,
) -> Result<Vec<u8>> {
    // TODO: Need to support converting receiver from hex address to bech32 here.

    let ret = build_signed_single_msg_tx(
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
        private_key.key.clone(),
    )?;

    Ok(ret)
}

/// creates the signed transaction for cosmos
pub fn get_msg_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    msg: &CosmosSDKMsgRaw,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(tx_info.into(), msg.into(), private_key.key.clone())?;
    Ok(ret)
}

/// query account details from cosmos address
pub fn query_account_details(api_url: String, address: String) -> Result<String> {
    let account_details: RawRpcAccountResponse = get_account_details_blocking(&api_url, &address)?;
    Ok(serde_json::to_string(&account_details)?)
}

/// query account details info from cosmos address
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

/// query account balance from cosmos address and denom name
pub fn query_account_balance(grpc_url: String, address: String, denom: String) -> Result<String> {
    let balance = get_account_balance_blocking(&grpc_url, &address, &denom)?;

    Ok(serde_json::to_string(&balance)?)
}

/// broadcast the cosmos transaction
pub fn broadcast_tx(
    tendermint_rpc_url: String,
    raw_signed_tx: Vec<u8>,
) -> Result<ffi::CosmosTransactionReceiptRaw> {
    let resp = broadcast_tx_sync_blocking(&tendermint_rpc_url, raw_signed_tx)?;
    if 0 == resp.code {
        Ok(resp.into())
    } else {
        Err(anyhow!("{:?}", resp))
    }
}

// create Login Info by `msg`
/// all information from the EIP-4361 plaintext message:
/// https://eips.ethereum.org/EIPS/eip-4361
fn new_logininfo(msg: String) -> Result<Box<CppLoginInfo>> {
    let msg = siwe::Message::from_str(&msg)?;
    let logininfo = LoginInfo { msg };
    Ok(Box::new(CppLoginInfo { logininfo }))
}

impl CppLoginInfo {
    /// Sign Login Info
    /// constructs the plaintext message and signs it according to EIP-191
    /// (as per EIP-4361). The returned vector is a serialized recoverable signature
    /// (as used in Ethereum).
    pub fn sign_logininfo(&self, private_key: &PrivateKey) -> anyhow::Result<Vec<u8>> {
        Ok(self.logininfo.sign(&private_key.key)?)
    }

    /// Verify Login Info
    /// It verified the signature matches + also verifies the content of the message:
    /// - address in the message matches the address recovered from the signature
    /// - the time is valid
    /// ...
    /// NOTE: the server may still need to do extra verifications according to its needs
    /// (e.g. verify chain-id, nonce, uri + possibly fetch additional data associated
    /// with the given Ethereum address, such as ERC-20/ERC-721/ERC-1155 asset ownership)
    pub fn verify_logininfo(&self, signature: &[u8]) -> anyhow::Result<()> {
        // TODO Reuse runtime on blocking function
        let rt = tokio::runtime::Runtime::new()?;
        // FIXME: domain, nonce, timestamp
        Ok(rt.block_on(self.logininfo.verify(signature))?)
    }
}
fn convert_amount(
    amount: &str,
    amount_unit: ffi::EthAmount,
) -> Result<defi_wallet_core_common::EthAmount> {
    match amount_unit {
        ffi::EthAmount::WeiDecimal => Ok(defi_wallet_core_common::EthAmount::WeiDecimal {
            amount: amount.to_string(),
        }),
        ffi::EthAmount::GweiDecimal => Ok(defi_wallet_core_common::EthAmount::GweiDecimal {
            amount: amount.to_string(),
        }),
        ffi::EthAmount::EthDecimal => Ok(defi_wallet_core_common::EthAmount::EthDecimal {
            amount: amount.to_string(),
        }),
        _ => Err(anyhow!("invalid coin unit, use correct enum for coin unit")),
    }
}

impl From<ffi::EthTxInfoRaw> for EthTxInfo {
    fn from(info: ffi::EthTxInfoRaw) -> Self {
        EthTxInfo {
            to_address: info.to_address,
            amount: convert_amount(&info.amount, info.amount_unit).unwrap(),
            nonce: info.nonce,
            gas_limit: info.gas_limit,
            gas_price: convert_amount(&info.gas_price, info.gas_price_unit).unwrap(),
            data: Some(info.data),
            legacy_tx: false,
        }
    }
}

/// sign cronos tx with private key
pub fn build_eth_signed_tx(
    tx_info: ffi::EthTxInfoRaw,
    network: &str,
    private_key: &PrivateKey,
) -> Result<Vec<u8>> {
    let signedtx = defi_wallet_core_common::build_signed_eth_tx(
        tx_info.into(),
        EthNetwork::Known {
            name: network.into(),
        },
        private_key.key.clone(),
    )?;
    Ok(signedtx)
}

/// builds an signed ethereum transaction given the inputs and signature
pub fn build_signed_eth_tx_with_signature(
    tx_info: ffi::EthTxInfoRaw,
    network: &str,
    from_address: &str,
    signature_bytes: [u8; 65], // 65 bytes
) -> Result<Vec<u8>> {
    let signature = Signature::try_from(&signature_bytes[..])?;

    let signedtx = defi_wallet_core_common::build_signed_eth_tx_with_signature(
        tx_info.into(),
        EthNetwork::Known {
            name: network.into(),
        },
        from_address,
        &signature,
    )?;
    Ok(signedtx)
}

/// sign cronos tx with private key in custom network
pub fn build_custom_eth_signed_tx(
    tx_info: ffi::EthTxInfoRaw,
    chain_id: u64,
    legacy: bool,
    private_key: &PrivateKey,
) -> Result<Vec<u8>> {
    let signedtx = defi_wallet_core_common::build_signed_eth_tx(
        tx_info.into(),
        EthNetwork::Custom { chain_id, legacy },
        private_key.key.clone(),
    )?;
    Ok(signedtx)
}

/// Returns the corresponding account's native token balance
/// formatted in _ETH decimals_ (e.g. "1.50000...") wrapped as string
pub fn get_eth_balance(address: &str, api_url: &str) -> Result<ffi::U256> {
    // TODO Reuse runtime on blocking function
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    let res = rt.block_on(defi_wallet_core_common::get_eth_balance(address, api_url))?;
    Ok(res.into())
}

/// Returns the corresponding account's nonce / number of transactions
/// sent from it.
pub fn get_eth_nonce(address: &str, api_url: &str) -> Result<String> {
    let res = defi_wallet_core_common::get_eth_transaction_count_blocking(address, api_url)?;
    // convert res to string
    Ok(res.to_string())
}

/// broadcast signed cronos tx
pub fn broadcast_eth_signed_raw_tx(
    raw_tx: Vec<u8>,
    web3api_url: &str,
    polling_interval_ms: u64,
) -> Result<CronosTransactionReceiptRaw> {
    let res = defi_wallet_core_common::broadcast_eth_signed_raw_tx_blocking(
        raw_tx,
        web3api_url,
        polling_interval_ms,
    )?;
    Ok(res.into())
}

/// create cronos tx info to sign
pub fn new_eth_tx_info() -> ffi::EthTxInfoRaw {
    ffi::EthTxInfoRaw {
        to_address: "".to_string(),
        amount: "0".to_string(),
        amount_unit: ffi::EthAmount::EthDecimal,
        nonce: "0".to_string(),
        gas_limit: "21000".to_string(),
        gas_price: "7".to_string(),
        gas_price_unit: ffi::EthAmount::WeiDecimal,
        data: vec![],
    }
}

pub fn set_cronos_httpagent(agent: &str) -> Result<()> {
    set_ethers_httpagent(agent)?;
    Ok(())
}
