use super::nft::*;
use crate::SecretKey;
#[cfg(not(target_arch = "wasm32"))]
use crate::UniffiCustomTypeConverter;
use cosmrs::bank::MsgSend;
use cosmrs::bip32::secp256k1::ecdsa::SigningKey;
use cosmrs::bip32::{PrivateKey, PublicKey, PublicKeyBytes, KEY_SIZE};
use cosmrs::crypto::{self, secp256k1::VerifyingKey};
use cosmrs::distribution::{MsgSetWithdrawAddress, MsgWithdrawDelegatorReward};
use cosmrs::staking::{MsgBeginRedelegate, MsgDelegate, MsgUndelegate};
use cosmrs::tx::{self, Fee, Msg, Raw, SignDoc, SignerInfo};
use cosmrs::{AccountId, Any, Coin, ErrorReport};
use eyre::{eyre, Context};
use ibc::applications::ics20_fungible_token_transfer::msgs::transfer::MsgTransfer;
use ibc::core::ics24_host::identifier::{ChannelId, PortId};
use ibc::signer::Signer;
use ibc::timestamp::Timestamp;
use ibc::tx_msg::Msg as IbcMsg;
use ibc::Height;
use ibc_proto::cosmos::base::v1beta1::Coin as IbcCoin;
use std::str::FromStr;
use std::sync::Arc;

/// human-readable bech32 prefix for Crypto.org Chain accounts
pub const CRYPTO_ORG_BECH32_HRP: &str = "cro";
/// human-readable bech32 prefix for Crypto.org Chain testnet accounts
pub const CRYPTO_ORG_TESTNET_BECH32_HRP: &str = "tcro";
/// human-readable bech32 prefix for Cronos accounts
pub const CRONOS_BECH32_HRP: &str = "crc";
/// human-readable bech32 prefix for Cosmos Hub accounts
pub const COSMOS_BECH32_HRP: &str = "cosmos";
/// mainnet chain id of Crypto.org Chain
pub const CRYPTO_ORG_CHAIN_ID: &str = "crypto-org-chain-mainnet-1";
/// testnet chain id of Crypto.org Chain Croeseid
pub const CRYPTO_ORG_CHAIN_TESTNET_ID: &str = "testnet-croeseid-4";
/// mainnet chain id of Cronos
pub const CRONOS_CHAIN_ID: &str = "cronosmainnet_25-1";
/// mainnet chain id of Cosmos Hub
pub const COSMOS_CHAIN_ID: &str = "cosmoshub-4";

/// Network to work with
pub enum Network {
    /// Crypto.org Chain mainnet
    CryptoOrgMainnet,
    /// Crypto.org Chain testnet
    CryptoOrgTestnet,
    /// Cronos mainnet beta
    CronosMainnet,
    /// Cosmos Hub mainnet
    CosmosHub,
    /// other network
    Other {
        /// Tendermint Chain Id
        chain_id: String,
        /// HD wallet coin type
        coin_type: u32,
        /// bech32 human-readable prefix
        bech32hrp: String,
    },
}

impl Network {
    /// return the network HD coin type
    pub fn get_coin_type(&self) -> u32 {
        match self {
            Network::CryptoOrgMainnet => 394,
            Network::CronosMainnet => 60,
            Network::CosmosHub => 118,
            Network::Other { coin_type, .. } => *coin_type,
            Network::CryptoOrgTestnet => 1,
        }
    }

    /// get the bech32 human-readable prefix
    pub fn get_bech32_hrp(&self) -> &str {
        match self {
            Network::CryptoOrgMainnet => CRYPTO_ORG_BECH32_HRP,
            Network::CronosMainnet => CRONOS_BECH32_HRP,
            Network::CosmosHub => COSMOS_BECH32_HRP,
            Network::Other { bech32hrp, .. } => bech32hrp,
            Network::CryptoOrgTestnet => CRYPTO_ORG_TESTNET_BECH32_HRP,
        }
    }

    fn get_chain_id(&self) -> eyre::Result<tendermint::chain::Id> {
        let chain_id = match self {
            Network::CryptoOrgMainnet => CRYPTO_ORG_CHAIN_ID,
            Network::CronosMainnet => CRONOS_CHAIN_ID,
            Network::CosmosHub => COSMOS_CHAIN_ID,
            Network::Other { chain_id, .. } => chain_id,
            Network::CryptoOrgTestnet => CRYPTO_ORG_CHAIN_TESTNET_ID,
        };
        chain_id.parse().context("invalid chain id")
    }
}

/// single coin amount
pub enum SingleCoin {
    /// basecro
    BaseCRO { amount: u64 },
    /// 1 CRO = 10^8 basecro on Crypto.org Chain mainnet OR 10^18 on Cronos/EVM
    CRO { amount: u64, network: Network },
    /// basecro
    TestnetBaseCRO { amount: u64 },
    /// 1 TCRO = 10^8 basetcro
    TestnetCRO { amount: u64 },
    /// uatom
    UATOM { amount: u64 },
    /// 1 ATOM = 10^6 uatom
    ATOM { amount: u64 },
    /// other coin unit
    Other { amount: String, denom: String },
}

impl TryInto<Coin> for &SingleCoin {
    type Error = ErrorReport;

    fn try_into(self) -> eyre::Result<Coin> {
        match self {
            SingleCoin::BaseCRO { amount } => Ok(Coin {
                amount: (*amount).into(),
                denom: "basecro".parse()?,
            }),
            SingleCoin::TestnetBaseCRO { amount } => Ok(Coin {
                amount: (*amount).into(),
                denom: "basetcro".parse()?,
            }),
            SingleCoin::TestnetCRO { amount } => {
                let base_amount = amount
                    .checked_mul(10 ^ 8)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(Coin {
                    amount: base_amount.into(),
                    denom: "basetcro".parse()?,
                })
            }
            SingleCoin::CRO { amount, network } => {
                let decimals = match network {
                    Network::CronosMainnet => 10 ^ 18,
                    _ => 10 ^ 8,
                };
                // FIXME: convert to Decimal when it supports multiplication
                let base_amount = amount
                    .checked_mul(decimals)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(Coin {
                    amount: base_amount.into(),
                    denom: "basecro".parse()?,
                })
            }
            SingleCoin::UATOM { amount } => Ok(Coin {
                amount: (*amount).into(),
                denom: "uatom".parse()?,
            }),
            SingleCoin::ATOM { amount } => {
                let base_amount = amount
                    .checked_mul(1_000_000)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(Coin {
                    amount: base_amount.into(),
                    denom: "uatom".parse()?,
                })
            }
            SingleCoin::Other { amount, denom } => Ok(Coin {
                amount: amount.parse()?,
                denom: denom.parse()?,
            }),
        }
    }
}

impl TryInto<IbcCoin> for &SingleCoin {
    type Error = ErrorReport;

    fn try_into(self) -> eyre::Result<IbcCoin> {
        match self {
            SingleCoin::BaseCRO { amount } => Ok(IbcCoin {
                amount: amount.to_string(),
                denom: "basecro".to_owned(),
            }),
            SingleCoin::TestnetBaseCRO { amount } => Ok(IbcCoin {
                amount: amount.to_string(),
                denom: "basetcro".to_owned(),
            }),
            SingleCoin::TestnetCRO { amount } => {
                let base_amount = amount
                    .checked_mul(10 ^ 8)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(IbcCoin {
                    amount: base_amount.to_string(),
                    denom: "basetcro".to_owned(),
                })
            }
            SingleCoin::CRO { amount, network } => {
                let decimals = match network {
                    Network::CronosMainnet => 10 ^ 18,
                    _ => 10 ^ 8,
                };
                // FIXME: convert to Decimal when it supports multiplication
                let base_amount = amount
                    .checked_mul(decimals)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(IbcCoin {
                    amount: base_amount.to_string(),
                    denom: "basecro".to_owned(),
                })
            }
            SingleCoin::UATOM { amount } => Ok(IbcCoin {
                amount: amount.to_string(),
                denom: "uatom".to_owned(),
            }),
            SingleCoin::ATOM { amount } => {
                let base_amount = amount
                    .checked_mul(1_000_000)
                    .ok_or_else(|| eyre!("integer overflow"))?;
                Ok(IbcCoin {
                    amount: base_amount.to_string(),
                    denom: "uatom".to_owned(),
                })
            }
            SingleCoin::Other { amount, denom } => Ok(IbcCoin {
                amount: amount.to_owned(),
                denom: denom.to_owned(),
            }),
        }
    }
}

/// wrapper around 33-byte secp256k1 public key
/// FIXME: investigate wrapping directly `cosmrs::crypto::PublicKey`
pub struct PublicKeyBytesWrapper(pub Vec<u8>);

/// unwrapping public key errors
/// FIXME: additional errors after wrapping directly `cosmrs::crypto::PublicKey`
#[derive(Debug, thiserror::Error)]
pub enum PublicKeyBytesError {
    #[error("The length should be 33-bytes")]
    InvalidLength,
}

/// size of the secp256k1 public key in the compressed form
pub const COMPRESSED_SECP256K1_PUBKEY_SIZE: usize = KEY_SIZE + 1;

impl From<PublicKeyBytesWrapper> for PublicKeyBytes {
    fn from(wrapper: PublicKeyBytesWrapper) -> Self {
        let mut result = [0u8; COMPRESSED_SECP256K1_PUBKEY_SIZE];
        result.copy_from_slice(&wrapper.0);
        result
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl UniffiCustomTypeConverter for PublicKeyBytesWrapper {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        if val.len() != COMPRESSED_SECP256K1_PUBKEY_SIZE {
            Err(PublicKeyBytesError::InvalidLength.into())
        } else {
            Ok(PublicKeyBytesWrapper(val))
        }
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

/// the common transaction data needed for Cosmos SDK transactions
pub struct CosmosSDKTxInfo {
    /// global account number of the sender
    pub account_number: u64,
    /// equivalent of "account nonce"
    pub sequence_number: u64,
    /// the maximum gas limit
    pub gas_limit: u64,
    /// the fee to be paid (gas_limit * gas_price)
    pub fee_amount: SingleCoin,
    /// transaction timeout
    pub timeout_height: u32,
    /// optional memo
    pub memo_note: Option<String>,
    /// the network to use
    pub network: Network,
}

/// Cosmos SDK message types
pub enum CosmosSDKMsg {
    /// MsgSend
    BankSend {
        /// recipient address in bech32
        recipient_address: String,
        /// amount to send
        amount: SingleCoin,
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
        amount: SingleCoin,
    },
    /// MsgDelegate
    StakingDelegate {
        /// validator address in bech32
        validator_address: String,
        /// amount to delegate
        amount: SingleCoin,
    },
    /// MsgUndelegate
    StakingUndelegate {
        /// validator address in bech32
        validator_address: String,
        /// amount to undelegate
        amount: SingleCoin,
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
        token: SingleCoin,
        /// Timeout height relative to the current block height.
        /// The timeout is disabled when set to 0.
        timeout_height: Height,
        /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
        /// The timeout is disabled when set to 0.
        timeout_timestamp: u64,
    },
}

impl CosmosSDKMsg {
    fn to_any(&self, sender_address: AccountId) -> eyre::Result<Any> {
        match self {
            CosmosSDKMsg::BankSend {
                recipient_address,
                amount,
            } => {
                let amount_coin: Coin = amount.try_into()?;
                let recipient_account_id = recipient_address.parse::<AccountId>()?;
                let msg_send = MsgSend {
                    from_address: sender_address,
                    to_address: recipient_account_id,
                    amount: vec![amount_coin],
                };
                msg_send.to_any()
            }
            CosmosSDKMsg::NftIssueDenom { id, name, schema } => {
                let msg_send = MsgIssueDenom {
                    id: id.parse::<DenomId>()?,
                    name: name.parse::<DenomName>()?,
                    schema: schema.to_owned(),
                    sender: sender_address,
                };
                msg_send.to_any()
            }
            CosmosSDKMsg::NftMint {
                id,
                denom_id,
                name,
                uri,
                data,
                recipient,
            } => {
                let recipient_account_id = recipient.parse::<AccountId>()?;
                let msg_send = MsgMintNft {
                    id: id.parse::<TokenId>()?,
                    denom_id: denom_id.parse::<DenomId>()?,
                    name: name.to_owned(),
                    uri: uri.parse::<TokenUri>()?,
                    data: data.to_owned(),
                    sender: sender_address,
                    recipient: recipient_account_id,
                };
                msg_send.to_any()
            }
            CosmosSDKMsg::NftEdit {
                id,
                denom_id,
                name,
                uri,
                data,
            } => {
                let msg_send = MsgEditNft {
                    id: id.parse::<TokenId>()?,
                    denom_id: denom_id.parse::<DenomId>()?,
                    name: name.to_owned(),
                    uri: uri.parse::<TokenUri>()?,
                    data: data.to_owned(),
                    sender: sender_address,
                };
                msg_send.to_any()
            }
            CosmosSDKMsg::NftTransfer {
                id,
                denom_id,
                recipient,
            } => {
                let recipient_account_id = recipient.parse::<AccountId>()?;
                let msg_send = MsgTransferNft {
                    id: id.parse::<TokenId>()?,
                    denom_id: denom_id.parse::<DenomId>()?,
                    sender: sender_address,
                    recipient: recipient_account_id,
                };
                msg_send.to_any()
            }
            CosmosSDKMsg::NftBurn { id, denom_id } => {
                let msg_send = MsgBurnNft {
                    id: id.parse::<TokenId>()?,
                    denom_id: denom_id.parse::<DenomId>()?,
                    sender: sender_address,
                };
                msg_send.to_any()
            }
            CosmosSDKMsg::StakingBeginRedelegate {
                validator_src_address,
                validator_dst_address,
                amount,
            } => {
                let validator_src_address = validator_src_address.parse::<AccountId>()?;
                let validator_dst_address = validator_dst_address.parse::<AccountId>()?;
                let amount: Coin = amount.try_into()?;

                let msg = MsgBeginRedelegate {
                    delegator_address: sender_address,
                    validator_src_address,
                    validator_dst_address,
                    amount,
                };
                msg.to_any()
            }
            CosmosSDKMsg::StakingDelegate {
                validator_address,
                amount,
            } => {
                let validator_address = validator_address.parse::<AccountId>()?;
                let amount: Coin = amount.try_into()?;

                let msg = MsgDelegate {
                    delegator_address: sender_address,
                    validator_address,
                    amount,
                };
                msg.to_any()
            }
            CosmosSDKMsg::StakingUndelegate {
                validator_address,
                amount,
            } => {
                let validator_address = validator_address.parse::<AccountId>()?;
                let amount: Coin = amount.try_into()?;

                let msg = MsgUndelegate {
                    delegator_address: sender_address,
                    validator_address,
                    amount,
                };
                msg.to_any()
            }
            CosmosSDKMsg::DistributionSetWithdrawAddress { withdraw_address } => {
                let withdraw_address = withdraw_address.parse::<AccountId>()?;

                let msg = MsgSetWithdrawAddress {
                    delegator_address: sender_address,
                    withdraw_address,
                };
                msg.to_any()
            }
            CosmosSDKMsg::DistributionWithdrawDelegatorReward { validator_address } => {
                let validator_address = validator_address.parse::<AccountId>()?;

                let msg = MsgWithdrawDelegatorReward {
                    delegator_address: sender_address,
                    validator_address,
                };
                msg.to_any()
            }
            CosmosSDKMsg::IbcTransfer {
                receiver,
                source_port,
                source_channel,
                token,
                timeout_height,
                timeout_timestamp,
            } => Ok(MsgTransfer {
                sender: Signer::new(sender_address),
                receiver: Signer::new(receiver),
                source_port: PortId::from_str(source_port)?,
                source_channel: ChannelId::from_str(source_channel)?,
                token: Some(token.try_into()?),
                // TODO: timeout_height and timeout_timestamp cannot both be 0.
                timeout_height: *timeout_height,
                timeout_timestamp: Timestamp::from_nanoseconds(*timeout_timestamp)?,
            }
            .to_any()),
        }
    }
}

fn get_msg_signdoc(
    tx_info: CosmosSDKTxInfo,
    msgs: Vec<CosmosSDKMsg>,
    sender_public_key: crypto::PublicKey,
) -> eyre::Result<SignDoc> {
    let chain_id = tx_info.network.get_chain_id()?;
    let sender_account_id = sender_public_key.account_id(tx_info.network.get_bech32_hrp())?;

    let mut msgs_any: Vec<Any> = Vec::new();
    for (_, value) in msgs.iter().enumerate() {
        msgs_any.push(value.to_any(sender_account_id.clone())?);
    }

    let tx_body = tx::Body::new(
        msgs_any,
        tx_info.memo_note.unwrap_or_default(),
        tx_info.timeout_height,
    );
    let signer_info = SignerInfo::single_direct(Some(sender_public_key), tx_info.sequence_number);
    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(
        (&tx_info.fee_amount).try_into()?,
        tx_info.gas_limit,
    ));

    SignDoc::new(&tx_body, &auth_info, &chain_id, tx_info.account_number)
}

fn get_signed_msg_tx(
    tx_info: CosmosSDKTxInfo,
    msgs: Vec<CosmosSDKMsg>,
    sender_private_key: SigningKey,
) -> eyre::Result<Raw> {
    let sender_pubkey = crypto::PublicKey::from(sender_private_key.public_key());
    let sign_doc = get_msg_signdoc(tx_info, msgs, sender_pubkey)?;
    sign_doc.sign(&cosmrs::crypto::secp256k1::SigningKey::new(Box::new(
        sender_private_key,
    )))
}

/// UniFFI 0.15.2 doesn't support external types for Kotlin yet
#[derive(Debug, thiserror::Error)]
pub enum ErrorWrapper {
    #[error("Error: {report}")]
    EyreReport { report: eyre::Report },
    #[error("Public key error: {0}")]
    PubkeyError(cosmrs::bip32::Error),
}

/// creates the transaction signing payload (`SignDoc`)
/// with a single Cosmos SDK message
pub fn get_single_msg_sign_payload(
    tx_info: CosmosSDKTxInfo,
    msg: CosmosSDKMsg,
    sender_pubkey: PublicKeyBytesWrapper,
) -> Result<Vec<u8>, ErrorWrapper> {
    get_msg_sign_payload(tx_info, vec![msg], sender_pubkey)
}

/// creates the signed transaction
/// with a single Cosmos SDK message
pub fn build_signed_single_msg_tx(
    tx_info: CosmosSDKTxInfo,
    msg: CosmosSDKMsg,
    secret_key: Arc<SecretKey>,
) -> Result<Vec<u8>, ErrorWrapper> {
    let raw_signed_tx = get_signed_msg_tx(tx_info, vec![msg], secret_key.get_signing_key())
        .map_err(|report| ErrorWrapper::EyreReport { report })?;
    raw_signed_tx
        .to_bytes()
        .map_err(|report| ErrorWrapper::EyreReport { report })
}

/// creates the transaction signing payload (`SignDoc`)
/// with some Cosmos SDK messages
pub fn get_msg_sign_payload(
    tx_info: CosmosSDKTxInfo,
    msgs: Vec<CosmosSDKMsg>,
    sender_pubkey: PublicKeyBytesWrapper,
) -> Result<Vec<u8>, ErrorWrapper> {
    let sender_public_key: crypto::PublicKey = crypto::PublicKey::from(
        VerifyingKey::from_bytes(sender_pubkey.into()).map_err(ErrorWrapper::PubkeyError)?,
    );
    get_msg_signdoc(tx_info, msgs, sender_public_key)
        .and_then(|doc| doc.into_bytes())
        .map_err(|report| ErrorWrapper::EyreReport { report })
}

/// creates the signed transaction
/// with some Cosmos SDK messages
pub fn build_signed_msg_tx(
    tx_info: CosmosSDKTxInfo,
    msgs: Vec<CosmosSDKMsg>,
    secret_key: Arc<SecretKey>,
) -> Result<Vec<u8>, ErrorWrapper> {
    let raw_signed_tx = get_signed_msg_tx(tx_info, msgs, secret_key.get_signing_key())
        .map_err(|report| ErrorWrapper::EyreReport { report })?;
    raw_signed_tx
        .to_bytes()
        .map_err(|report| ErrorWrapper::EyreReport { report })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::*;
    use cosmrs::bank::MsgSend;
    use cosmrs::crypto::secp256k1::SigningKey;
    use cosmrs::proto;
    use cosmrs::Coin;
    use cosmrs::Tx;
    use prost::Message;

    const TX_INFO: CosmosSDKTxInfo = CosmosSDKTxInfo {
        account_number: 1,
        sequence_number: 0,
        gas_limit: 100_000,
        timeout_height: 9001,
        fee_amount: SingleCoin::ATOM { amount: 1 },
        memo_note: None,
        network: Network::CosmosHub,
    };

    const WORDS: &str = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief";

    #[test]
    fn signdoc_construction_works() {
        let sender_private_key = SigningKey::random();
        let sender_public_key = sender_private_key.public_key();

        let sign_doc_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::BankSend {
                recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                amount: SingleCoin::ATOM { amount: 1 },
            },
            PublicKeyBytesWrapper(sender_public_key.to_bytes()),
        )
        .expect("ok sign doc");
        assert!(proto::cosmos::tx::v1beta1::SignDoc::decode(&*sign_doc_raw).is_ok());
    }

    #[test]
    fn signdoc_construction_works_mutimsg() {
        let sender_private_key = SigningKey::random();
        let sender_public_key = sender_private_key.public_key();
        let mut msgs = Vec::new();
        msgs.push(CosmosSDKMsg::BankSend {
            recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            amount: SingleCoin::ATOM { amount: 1 },
        });

        msgs.push(CosmosSDKMsg::BankSend {
            recipient_address: "cosmos1a83x94xww47e32rgpytttucx2vexxcn2lc2ekx".to_string(),
            amount: SingleCoin::ATOM { amount: 2 },
        });

        let sign_doc_raw = get_msg_sign_payload(
            TX_INFO,
            msgs,
            PublicKeyBytesWrapper(sender_public_key.to_bytes()),
        )
        .expect("ok sign doc");
        assert!(proto::cosmos::tx::v1beta1::SignDoc::decode(&*sign_doc_raw).is_ok());
    }

    #[test]
    fn signing_works() {
        let secret_key = SecretKey::new();

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::BankSend {
                recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                amount: SingleCoin::ATOM { amount: 1 },
            },
            Arc::new(secret_key),
        )
        .expect("ok signed tx");
        assert!(Tx::from_bytes(&tx_raw).is_ok());
    }

    #[test]
    fn signing_works_mutimsg() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");
        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let mut msgs = Vec::new();

        msgs.push(CosmosSDKMsg::BankSend {
            recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            amount: SingleCoin::ATOM { amount: 1 },
        });

        msgs.push(CosmosSDKMsg::BankSend {
            recipient_address: "cosmos1a83x94xww47e32rgpytttucx2vexxcn2lc2ekx".to_string(),
            amount: SingleCoin::ATOM { amount: 2 },
        });

        let tx_raw = build_signed_msg_tx(TX_INFO, msgs, private_key).expect("ok signed tx");
        assert_eq!(
            hex::encode(tx_raw),
            "0aa9020a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a1a100a057561746f6d1207313030303030300a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73316138337839347877773437653332726770797474747563783276657878636e326c6332656b781a100a057561746f6d12073230303030303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a406be1c153eda9e3ba022d2e9138c0682991ba6cf6b8b7bdc75ae1adb88b8a977b35e18292b569cb66ffff16189f37a5848648f14caa1084cfb4f7041deda737ae"
        );
    }

    use crate::wallet::HDWallet;
    use ethers::utils::hex;

    #[test]
    fn signing_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let keystr = hex::encode(private_key.get_signing_key().to_bytes());
        assert_eq!(
            keystr,
            "cbdff41bb60c39f7b85d6378586951f61cf1e8a33c0a034b1f9f98ffe3ad18cf"
        );

        let pubkeys = private_key.get_public_key_bytes();
        assert_eq!(
            hex::encode(pubkeys),
            "028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c520"
        );

        let cosmos_address = wallet
            .get_address(
                WalletCoin::CosmosSDK {
                    network: Network::CosmosHub,
                },
                0,
            )
            .expect("address");
        assert_eq!(
            cosmos_address,
            "cosmos1l5s7tnj28a7zxeeckhgwlhjys8dlrrefgqr4pj"
        );

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::BankSend {
                recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                amount: SingleCoin::ATOM { amount: 1 },
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0a96010a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a1a100a057561746f6d12073130303030303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::BankSend {
                recipient_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                amount: SingleCoin::ATOM { amount: 1 },
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0a96010a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a1a100a057561746f6d12073130303030303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40aa554d4be2ac72d644002296882c188de39944efd21fc021bf1202721fff40d05e9c86d398b11bb94e16cf79dd4866eca22d84b6785bd0098ed353615585485c"
        );
    }

    #[test]
    fn staking_delegate_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::StakingDelegate {
                validator_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
                amount: SingleCoin::UATOM { amount: 100 },
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0aa0010a9a010a232f636f736d6f732e7374616b696e672e763162657461312e4d736744656c656761746512730a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b331a0c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::StakingDelegate {
                validator_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
                amount: SingleCoin::UATOM { amount: 100 },
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0aa0010a9a010a232f636f736d6f732e7374616b696e672e763162657461312e4d736744656c656761746512730a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b331a0c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a404d71f59fb847a319b5cd4a831eed8c9baa4051a656392be6c981f95d5debf552011318ac433caf47e8df57d6fb133cf9f5d91db031dff59beb2d98b7e041a125"
        );
    }

    #[test]
    fn staking_undelegate_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::StakingUndelegate {
                validator_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
                amount: SingleCoin::UATOM { amount: 100 },
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0aa2010a9c010a252f636f736d6f732e7374616b696e672e763162657461312e4d7367556e64656c656761746512730a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b331a0c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::StakingUndelegate {
                validator_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
                amount: SingleCoin::UATOM { amount: 100 },
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0aa2010a9c010a252f636f736d6f732e7374616b696e672e763162657461312e4d7367556e64656c656761746512730a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b331a0c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a407c468b64e58510b3dc20259d6042f280b8ee9e9aca6a0b3bfc21d931509659b70169aad7543970b65c8bc6aa3bccbb8868ce85d3eece042396492e6dc666404a"
        );
    }

    #[test]
    fn staking_begin_redelegate_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::StakingBeginRedelegate {
                validator_src_address: "cosmosvaloper1l5s7tnj28a7zxeeckhgwlhjys8dlrrefd5hqdp"
                    .to_string(),
                validator_dst_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
                amount: SingleCoin::UATOM { amount: 100 },
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0ade010ad8010a2a2f636f736d6f732e7374616b696e672e763162657461312e4d7367426567696e526564656c656761746512a9010a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572316c357337746e6a323861377a786565636b6867776c686a797338646c727265666435687164701a34636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b33220c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::StakingBeginRedelegate {
                validator_src_address: "cosmosvaloper1l5s7tnj28a7zxeeckhgwlhjys8dlrrefd5hqdp"
                    .to_string(),
                validator_dst_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
                amount: SingleCoin::UATOM { amount: 100 },
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0ade010ad8010a2a2f636f736d6f732e7374616b696e672e763162657461312e4d7367426567696e526564656c656761746512a9010a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572316c357337746e6a323861377a786565636b6867776c686a797338646c727265666435687164701a34636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b33220c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40de252fd4e12b786c499d62ea5cc7070899acff3b88d6438c5542529a4a18d15755496029a1936865658b872ec9765d92a8394bad2443da84e73536917a65139f"
        );
    }

    #[test]
    fn distribution_setwithdraw_address_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::DistributionSetWithdrawAddress {
                withdraw_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0a9a010a94010a322f636f736d6f732e646973747269627574696f6e2e763162657461312e4d7367536574576974686472617741646472657373125e0a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::DistributionSetWithdrawAddress {
                withdraw_address: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0a9a010a94010a322f636f736d6f732e646973747269627574696f6e2e763162657461312e4d7367536574576974686472617741646472657373125e0a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40c29ab82aec56651fb33a4df92f499bb4624d0be31cd51d64df234a4d380282bb5ebda7aa54a84d8075f6b2ffb0b5fa5f98118b108888fcfdbbaf4efaca4ffdba"
        );
    }

    #[test]
    fn distribution_withdraw_delegator_reward_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::DistributionWithdrawDelegatorReward {
                validator_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0aa6010aa0010a372f636f736d6f732e646973747269627574696f6e2e763162657461312e4d7367576974686472617744656c656761746f7252657761726412650a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b3318a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::DistributionWithdrawDelegatorReward {
                validator_address: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3"
                    .to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0aa6010aa0010a372f636f736d6f732e646973747269627574696f6e2e763162657461312e4d7367576974686472617744656c656761746f7252657761726412650a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b3318a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40ae166e9cc8489ded5e6dc82e99d0b7ee017fc0234a70c0851cff133c811e92165391c5404c474278ed8cbe85b28f1cf4ee6e59071ccdf3d495dddfd12c4029f1"
        );
    }

    #[test]
    fn nft_issue_denom_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::NftIssueDenom {
                id: "edition01".to_string(),
                name: "domingo1".to_string(),
                schema: "test".to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0a720a6d0a1f2f636861696e6d61696e2e6e66742e76312e4d7367497373756544656e6f6d124a0a0965646974696f6e30311208646f6d696e676f311a0474657374222d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::NftIssueDenom {
                id: "edition01".to_string(),
                name: "domingo1".to_string(),
                schema: "test".to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0a720a6d0a1f2f636861696e6d61696e2e6e66742e76312e4d7367497373756544656e6f6d124a0a0965646974696f6e30311208646f6d696e676f311a0474657374222d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a404d0eb09d0735c80d8dfa9a7113beeff4dc38fb6f6bdfcad1a39ff0153ba5eaa3236d8413abcd31c62755946238656b80df428c7d05b43fcff3531dfae7687064"
        );
    }

    #[test]
    fn nft_transfer_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::NftTransfer {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
                recipient: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0a9d010a97010a202f636861696e6d61696e2e6e66742e76312e4d73675472616e736665724e465412730a0965646974696f6e30311208646f6d696e676f311a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a222d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::NftTransfer {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
                recipient: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0a9d010a97010a202f636861696e6d61696e2e6e66742e76312e4d73675472616e736665724e465412730a0965646974696f6e30311208646f6d696e676f311a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a222d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a409645a66de4809f282349fce4a80f8478d78b0b0c0d8d23f4ebe7430589fed7123e0e432f244e7b991130a475db8e2d5f90ae5f933682763afea798f78da156ff"
        );
    }

    #[test]
    fn nft_mint_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::NftMint {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
                name: "test".to_string(),
                uri: "test".to_string(),
                data: "test".to_string(),
                recipient: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0aac010aa6010a1c2f636861696e6d61696e2e6e66742e76312e4d73674d696e744e46541285010a0965646974696f6e30311208646f6d696e676f311a04746573742204746573742a0474657374322d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a3a2d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::NftMint {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
                name: "test".to_string(),
                uri: "test".to_string(),
                data: "test".to_string(),
                recipient: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0aac010aa6010a1c2f636861696e6d61696e2e6e66742e76312e4d73674d696e744e46541285010a0965646974696f6e30311208646f6d696e676f311a04746573742204746573742a0474657374322d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a3a2d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a401a3eb24123103ee0ec2856315311b8c9c01e3e54249beb18bec91864834c6ffd7605e2a866fa7307f2786bc15e9075fa8d73cd188924eb7bded6214c858f9fdf"
        );
    }

    #[test]
    fn nft_edit_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::NftEdit {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
                name: "test".to_string(),
                uri: "test".to_string(),
                data: "test".to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0a7b0a760a1c2f636861696e6d61696e2e6e66742e76312e4d7367456469744e465412560a0965646974696f6e30311208646f6d696e676f311a04746573742204746573742a0474657374322d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::NftEdit {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
                name: "test".to_string(),
                uri: "test".to_string(),
                data: "test".to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0a7b0a760a1c2f636861696e6d61696e2e6e66742e76312e4d7367456469744e465412560a0965646974696f6e30311208646f6d696e676f311a04746573742204746573742a0474657374322d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a401134c4d5d9c1c6f5435e2dcc701512401c4220249b54ffc7c0e6793311399e9d60207caf1c175cbfc6ab999c7d8e75ef5f66931f73829e03f1ea9d3987bf442e"
        );
    }

    #[test]
    fn nft_burn_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::NftBurn {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0a690a640a1c2f636861696e6d61696e2e6e66742e76312e4d73674275726e4e465412440a0965646974696f6e30311208646f6d696e676f311a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::NftBurn {
                id: "edition01".to_string(),
                denom_id: "domingo1".to_string(),
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0a690a640a1c2f636861696e6d61696e2e6e66742e76312e4d73674275726e4e465412440a0965646974696f6e30311208646f6d696e676f311a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a4046e4de5a3c55bd27c2e359315e9b52bb684cc0c3e9d470e77a4d922a1bf2c1b334b3504ce639cc94ed84f403f5af4878ae4efea3a696caf9da49597bed2717d9"
        );
    }

    #[test]
    fn ibc_transfer_check() {
        let wallet = HDWallet::recover_wallet(WORDS.to_string(), None).expect("wallet");

        let private_key = wallet
            .get_key("m/44'/118'/0'/0/0".to_string())
            .expect("key");

        let payload_raw = get_single_msg_sign_payload(
            TX_INFO,
            CosmosSDKMsg::IbcTransfer {
                source_channel: "channel-3".to_string(),
                source_port: "transfer".to_string(),
                receiver: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                token: SingleCoin::Other {
                    amount: "100000000".to_string(),
                    denom: "basetcro".to_string(),
                },
                timeout_height: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp: 1645800000000000000,
            },
            PublicKeyBytesWrapper(private_key.get_public_key_bytes()),
        )
        .expect("ok signed payload");

        assert_eq!(
            hex::encode(payload_raw),
            "0aca010ac4010a292f6962632e6170706c69636174696f6e732e7472616e736665722e76312e4d73675472616e736665721296010a087472616e7366657212096368616e6e656c2d331a150a08626173657463726f1209313030303030303030222d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a2a2d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a3200388080da9a95ccc3eb1618a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a0b636f736d6f736875622d342001"
        );

        let tx_raw = build_signed_single_msg_tx(
            TX_INFO,
            CosmosSDKMsg::IbcTransfer {
                source_channel: "channel-3".to_string(),
                source_port: "transfer".to_string(),
                receiver: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string(),
                token: SingleCoin::Other {
                    amount: "100000000".to_string(),
                    denom: "basetcro".to_string(),
                },
                timeout_height: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp: 1645800000000000000,
            },
            private_key,
        )
        .expect("ok signed tx");

        assert_eq!(
            hex::encode(tx_raw),
            "0aca010ac4010a292f6962632e6170706c69636174696f6e732e7472616e736665722e76312e4d73675472616e736665721296010a087472616e7366657212096368616e6e656c2d331a150a08626173657463726f1209313030303030303030222d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a2a2d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a3200388080da9a95ccc3eb1618a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a409cee761ef007f4e0020dc1fe85610affd7555227e15cd068a364659ed58b638e725f543da0e1c6e8d39076ea9400de778053650053cbf2c98f3f72499938b97d"
        );
    }

    #[test]
    fn message_check() {
        let amount = &SingleCoin::ATOM { amount: 1 };
        let amount_coin: Coin = amount.try_into().unwrap();
        let sender_address = &"cosmos1l5s7tnj28a7zxeeckhgwlhjys8dlrrefgqr4pj".to_string();
        let sender_account_id = sender_address.parse::<AccountId>().unwrap();
        let recipient_address = &"cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_string();
        let recipient_account_id = recipient_address.parse::<AccountId>().unwrap();
        let msg_send = MsgSend {
            from_address: sender_account_id,
            to_address: recipient_account_id,
            amount: vec![amount_coin],
        };

        assert_eq!(
            hex::encode(msg_send.to_any().unwrap_or_default().value),
            "0a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a1a100a057561746f6d120731303030303030"
        );
    }
}
