use anyhow::{anyhow, Result};
use defi_wallet_core_common::{
    build_signed_single_msg_tx, get_single_msg_sign_payload, CosmosSDKMsg, CosmosSDKTxInfo,
    HDWallet, Network, PublicKeyBytesWrapper, SecretKey, SingleCoin, WalletCoin,
    COMPRESSED_SECP256K1_PUBKEY_SIZE,
};
use std::sync::Arc;

/// Wrapper of CosmosSDKMsg
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
        }
    }
}

#[cxx::bridge(namespace = "org::defi_wallet_core")]
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

    extern "Rust" {
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
        fn new_wallet(password: String) -> Box<Wallet>;
        fn get_default_address(self: &Wallet, coin: CoinType) -> Result<String>;
        fn new_privatekey() -> Box<PrivateKey>;
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

pub struct PrivateKey {
    key: Arc<SecretKey>,
}

fn new_privatekey() -> Box<PrivateKey> {
    let ret = PrivateKey {
        key: Arc::new(SecretKey::new()),
    };
    Box::new(ret)
}

impl PrivateKey {}

pub struct Wallet {
    wallet: HDWallet,
}

fn new_wallet(password: String) -> Box<Wallet> {
    let ret = Wallet {
        wallet: HDWallet::generate_wallet(Some(password)),
    };
    Box::new(ret)
}

impl Wallet {
    pub fn get_default_address(&self, coin: CoinType) -> anyhow::Result<String> {
        Ok(self.wallet.get_default_address(coin.into())?)
    }

    pub fn get_key(&self, derivation_path: String) -> anyhow::Result<Box<PrivateKey>> {
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

fn get_msg_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    msg: &CosmosSDKMsgRaw,
) -> Result<Vec<u8>> {
    let ret = build_signed_single_msg_tx(tx_info.into(), msg.into(), private_key.key.clone())?;
    Ok(ret)
}
