use anyhow::{anyhow, Result};
use defi_wallet_core_common::{
    broadcast_tx_sync_blocking, build_signed_msg_tx, build_signed_single_msg_tx,
    get_account_balance_blocking, get_account_details_blocking, get_single_msg_sign_payload,
    BalanceApiVersion, CosmosSDKMsg, CosmosSDKTxInfo, CronosTxInfo, EthError, HDWallet, Height,
    LoginInfo, Network, PublicKeyBytesWrapper, RawRpcAccountResponse, SecretKey, SingleCoin,
    WalletCoin, COMPRESSED_SECP256K1_PUBKEY_SIZE,
};
use defi_wallet_core_common::{transaction, Client};
use defi_wallet_core_proto as proto;
use proto::chainmain::nft::v1::{BaseNft, Collection, Denom, IdCollection, Owner};
use std::str::FromStr;
use std::sync::Arc;

pub struct GrpcClient(Client);

/// Create a new grpc client
// It can only be defined outside the `impl GrpcClient`, otherwise the mod ffi can not find it
fn new_grpc_client(grpc_url: String) -> Result<Box<GrpcClient>> {
    let client = Client::new_blocking(grpc_url)?;
    Ok(Box::new(GrpcClient(client)))
}

impl GrpcClient {
    /// Supply queries the total supply of a given denom or owner
    pub fn supply(&self, denom_id: String, owner: String) -> Result<u64> {
        let supply = self.0.supply_blocking(denom_id, owner)?;
        Ok(supply)
    }

    /// Owner queries the NFTs of the specified owner
    pub fn owner(&self, denom_id: String, owner: String) -> Result<Box<OwnerRaw>> {
        let owner = self
            .0
            .owner_blocking(denom_id, owner)?
            .ok_or(anyhow::anyhow!("No Owner"))?;
        Ok(Box::new(owner.into()))
    }

    /// Collection queries the NFTs of the specified denom
    pub fn collection(&self, denom_id: String) -> Result<Box<CollectionRaw>> {
        let collection = self
            .0
            .collection_blocking(denom_id)?
            .ok_or(anyhow::anyhow!("No Collection"))?;
        Ok(Box::new(collection.into()))
    }

    /// Denom queries the definition of a given denom
    pub fn denom(&self, denom_id: String) -> Result<Box<DenomRaw>> {
        let denom = self
            .0
            .denom_blocking(denom_id)?
            .ok_or(anyhow::anyhow!("No denom"))?;
        Ok(Box::new(denom.into()))
    }

    /// DenomByName queries the definition of a given denom by name
    pub fn denom_by_name(&self, denom_name: String) -> Result<Box<DenomRaw>> {
        let denom = self
            .0
            .denom_by_name_blocking(denom_name)?
            .ok_or(anyhow::anyhow!("No denom"))?;
        Ok(Box::new(denom.into()))
    }

    /// Denoms queries all the denoms
    pub fn denoms(&self) -> Result<Vec<DenomRaw>> {
        let denoms = self.0.denoms_blocking()?;
        Ok(denoms.into_iter().map(|v| v.into()).collect())
    }

    /// NFT queries the NFT for the given denom and token ID
    pub fn nft(&self, denom_id: String, token_id: String) -> Result<Box<BaseNftRaw>> {
        let nft = self
            .0
            .nft_blocking(denom_id, token_id)?
            .ok_or(anyhow::anyhow!("No Nft"))?;
        Ok(Box::new(nft.into()))
    }
}

/// Wrapper of proto::chainmain::nft::v1::Denom
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct DenomRaw {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub creator: String,
}

impl From<Denom> for DenomRaw {
    fn from(d: Denom) -> DenomRaw {
        DenomRaw {
            id: d.id,
            name: d.name,
            schema: d.schema,
            creator: d.creator,
        }
    }
}

/// Wrapper of proto::chainmain::nft::v1::BaseNft
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct BaseNftRaw {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub data: String,
    pub owner: String,
}

impl From<BaseNft> for BaseNftRaw {
    fn from(d: BaseNft) -> BaseNftRaw {
        BaseNftRaw {
            id: d.id,
            name: d.name,
            uri: d.uri,
            data: d.data,
            owner: d.owner,
        }
    }
}

/// Wrapper of proto::chainmain::nft::v1::Owner
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct OwnerRaw {
    pub address: String,
    pub id_collections: Vec<IdCollection>,
}

impl From<Owner> for OwnerRaw {
    fn from(d: Owner) -> OwnerRaw {
        OwnerRaw {
            address: d.address,
            id_collections: d.id_collections,
        }
    }
}

/// Wrapper of proto::chainmain::nft::v1::Collection
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct CollectionRaw {
    pub denom: Option<Denom>,
    pub nfts: Vec<BaseNft>,
}

impl From<Collection> for CollectionRaw {
    fn from(d: Collection) -> CollectionRaw {
        CollectionRaw {
            denom: d.denom,
            nfts: d.nfts,
        }
    }
}

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

// wrapper for LoginInfo
pub struct CppLoginInfo {
    pub logininfo: LoginInfo,
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
    pub enum EthNetwork {
        EthereumMainnet,
        RopstenTestnet,
        KovanTestnet,
        RinkebyTestnet,
        GoerliTestnet,
        Cronos,
        CronosTestnet,
        Custom,
    }

    pub enum EthAmount {
        /// 10^-18 ETH
        WeiDecimal,
        /// 10^-9 ETH
        GweiDecimal,
        EthDecimal,
    }

    pub struct CronosTxInfoRaw {
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
        fn get_address(self: &Wallet, coin: CoinType, index: u32) -> Result<String>;
        fn get_eth_address(self: &Wallet, index: u32) -> Result<String>;
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
        type GrpcClient;
        fn new_grpc_client(grpc_url: String) -> Result<Box<GrpcClient>>;
        fn supply(self: &GrpcClient, denom_id: String, owner: String) -> Result<u64>;
        type OwnerRaw;
        pub fn owner(self: &GrpcClient, denom_id: String, owner: String) -> Result<Box<OwnerRaw>>;
        type CollectionRaw;
        pub fn collection(self: &GrpcClient, denom_id: String) -> Result<Box<CollectionRaw>>;
        type DenomRaw;
        pub fn denom(self: &GrpcClient, denom_id: String) -> Result<Box<DenomRaw>>;
        pub fn denom_by_name(self: &GrpcClient, denom_name: String) -> Result<Box<DenomRaw>>;
        fn denoms(self: &GrpcClient) -> Result<Vec<DenomRaw>>;
        type BaseNftRaw;
        fn nft(self: &GrpcClient, denom_id: String, token_id: String) -> Result<Box<BaseNftRaw>>;
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
        fn new_logininfo(msg: String) -> Result<Box<CppLoginInfo>>;
        fn sign_logininfo(self: &CppLoginInfo, private_key: &PrivateKey) -> Result<Vec<u8>>;
        fn verify_logininfo(self: &CppLoginInfo, signature: &[u8]) -> Result<Vec<u8>>;

        pub fn new_eth_tx_info() -> CronosTxInfoRaw;
        pub fn build_eth_signed_tx(
            tx_info: CronosTxInfoRaw,
            network: EthNetwork,
            custom_network_id: u64,
            secret_key: &PrivateKey,
        ) -> Result<Vec<u8>>;

        pub fn get_eth_balance(address: &str, api_url: &str) -> Result<String>;

        pub fn get_eth_nonce(address: &str, api_url: &str) -> Result<String>;

        pub fn broadcast_eth_signed_raw_tx(raw_tx: Vec<u8>, web3api_url: &str) -> Result<String>;
    } // end of RUST block
} // end of ffi block

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
            _ => WalletCoin::Ethereum,
        }
    }
}

impl From<ffi::EthNetwork> for defi_wallet_core_common::EthNetwork {
    fn from(network: ffi::EthNetwork) -> Self {
        match network {
            ffi::EthNetwork::EthereumMainnet => {
                defi_wallet_core_common::EthNetwork::EthereumMainnet
            }
            _ => defi_wallet_core_common::EthNetwork::RopstenTestnet,
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
        self.get_address(coin, 0)
    }

    pub fn get_address(&self, coin: CoinType, index: u32) -> Result<String> {
        Ok(self.wallet.get_address(coin.into(), index)?)
    }

    pub fn get_eth_address(&self, index: u32) -> Result<String> {
        self.get_address(CoinType::Ethereum, index)
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
    let ret = transaction::nft::get_nft_issue_denom_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        name,
        schema,
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
    let ret = transaction::nft::get_nft_mint_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
        name,
        uri,
        data,
        recipient,
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
    let ret = transaction::nft::get_nft_edit_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
        name,
        uri,
        data,
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
    let ret = transaction::nft::get_nft_transfer_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
        recipient,
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
    let ret = transaction::nft::get_nft_burn_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
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

fn new_logininfo(msg: String) -> Result<Box<CppLoginInfo>> {
    let msg = siwe::Message::from_str(&msg)?;
    let logininfo = LoginInfo { msg };
    Ok(Box::new(CppLoginInfo { logininfo }))
}

impl CppLoginInfo {
    pub fn sign_logininfo(&self, private_key: &PrivateKey) -> anyhow::Result<Vec<u8>> {
        let message = self.logininfo.msg.to_string();
        let secretkey = private_key.key.clone();
        let ret = secretkey
            .sign_eth(message.as_bytes(), self.logininfo.msg.chain_id)
            .map(|x| x.to_vec())?;
        Ok(ret)
    }

    pub fn verify_logininfo(&self, signature: &[u8]) -> anyhow::Result<Vec<u8>> {
        let sig: [u8; 65] = signature
            .try_into()
            .map_err(|_e| EthError::SignatureError)?;
        let result = self.logininfo.msg.verify(sig)?;
        Ok(result)
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
fn convert_network(
    network: ffi::EthNetwork,
    custom_network_id: u64,
) -> Result<defi_wallet_core_common::EthNetwork> {
    match network {
        ffi::EthNetwork::EthereumMainnet => {
            Ok(defi_wallet_core_common::EthNetwork::EthereumMainnet)
        }
        ffi::EthNetwork::RopstenTestnet => Ok(defi_wallet_core_common::EthNetwork::RopstenTestnet),
        ffi::EthNetwork::KovanTestnet => Ok(defi_wallet_core_common::EthNetwork::KovanTestnet),
        ffi::EthNetwork::RinkebyTestnet => Ok(defi_wallet_core_common::EthNetwork::RinkebyTestnet),
        ffi::EthNetwork::GoerliTestnet => Ok(defi_wallet_core_common::EthNetwork::GoerliTestnet),
        ffi::EthNetwork::Cronos => Ok(defi_wallet_core_common::EthNetwork::Cronos),
        ffi::EthNetwork::CronosTestnet => Ok(defi_wallet_core_common::EthNetwork::CronosTestnet),
        _ => Ok(defi_wallet_core_common::EthNetwork::Custom {
            chain_id: custom_network_id,
        }),
    }
}

impl From<ffi::CronosTxInfoRaw> for CronosTxInfo {
    fn from(info: ffi::CronosTxInfoRaw) -> Self {
        CronosTxInfo {
            to_address: info.to_address,
            amount: convert_amount(&info.amount, info.amount_unit).unwrap(),
            nonce: info.nonce,
            gas_limit: info.gas_limit,
            gas_price: convert_amount(&info.gas_price, info.gas_price_unit).unwrap(),
            data: Some(info.data),
        }
    }
}

/// sign cronos tx with private key
pub fn build_eth_signed_tx(
    tx_info: ffi::CronosTxInfoRaw,
    network: ffi::EthNetwork,
    custom_network_id: u64,
    private_key: &PrivateKey,
) -> Result<Vec<u8>> {
    let signedtx = defi_wallet_core_common::build_signed_eth_tx(
        tx_info.into(),
        convert_network(network, custom_network_id)?,
        private_key.key.clone(),
    )?;
    Ok(signedtx)
}

/// get balance from cronos node
pub fn get_eth_balance(address: &str, api_url: &str) -> Result<String> {
    let res = defi_wallet_core_common::get_eth_balance_blocking(address, api_url)?;
    Ok(res)
}

/// get nonce from cronos node , which transsaction count of the address
pub fn get_eth_nonce(address: &str, api_url: &str) -> Result<String> {
    let res = defi_wallet_core_common::get_eth_transaction_count_blocking(address, api_url)?;
    // convert res to string
    Ok(res.to_string())
}

/// broadcast signed cronos tx
pub fn broadcast_eth_signed_raw_tx(raw_tx: Vec<u8>, web3api_url: &str) -> Result<String> {
    let res = defi_wallet_core_common::broadcast_eth_signed_raw_tx_blocking(raw_tx, web3api_url)?;
    Ok(res)
}

/// create cronos tx info to sign
pub fn new_eth_tx_info() -> ffi::CronosTxInfoRaw {
    ffi::CronosTxInfoRaw {
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
