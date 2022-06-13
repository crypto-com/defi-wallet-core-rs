use crate::proto::chainmain;
use crate::transaction::cosmos_sdk::{CosmosError, SingleCoin};
use crate::transaction::nft::{
    DenomId, DenomName, MsgBurnNft, MsgEditNft, MsgIssueDenom, MsgMintNft, MsgTransferNft, TokenId,
    TokenUri,
};
use cosmos_sdk_proto::cosmos::{bank, distribution, staking};
use cosmrs::bank::MsgSend;
use cosmrs::distribution::{MsgSetWithdrawAddress, MsgWithdrawDelegatorReward};
use cosmrs::staking::{MsgBeginRedelegate, MsgDelegate, MsgUndelegate};
use cosmrs::tx::Msg;
use cosmrs::{AccountId, Any};
use ibc::applications::ics20_fungible_token_transfer::msgs::transfer::MsgTransfer;
use ibc::core::ics24_host::identifier::{ChannelId, PortId};
use ibc::signer::Signer;
use ibc::timestamp::Timestamp;
use ibc::tx_msg::Msg as IbcMsg;
use ibc::Height;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Cosmos raw message that is parsed from Protobuf or JSON.
/// FIXME:
/// Since `CosmosSDKMsg` is constructed by fields and has no `sender_address` which is a wallet
/// address. `CosmosRawMsg` is parsed directly from Protobuf or JSON, it should have the all fields
/// of original message.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CosmosRawMsg {
    /// Normal message
    Normal { msg: CosmosRawNormalMsg },
    /// `crypto.org` special message
    CryptoOrg { msg: CosmosRawCryptoOrgMsg },
    // TODO: Add messages of `LunaClassic` chain here.
    /// Any message
    /// It is only used for messages which has not been supported.
    Any { type_url: String, value: Vec<u8> },
}

impl CosmosRawMsg {
    pub fn to_any(&self) -> eyre::Result<Any> {
        match self {
            Self::Normal { msg } => msg.to_any(),
            Self::CryptoOrg { msg } => msg.to_any(),
            Self::Any { type_url, value } => Ok(cosmrs::Any {
                type_url: type_url.clone(),
                value: value.clone(),
            }),
        }
    }
}

impl From<bank::v1beta1::MsgSend> for CosmosRawMsg {
    fn from(msg: bank::v1beta1::MsgSend) -> Self {
        Self::Normal {
            msg: CosmosRawNormalMsg::BankSend {
                from_address: msg.from_address,
                to_address: msg.to_address,
                amount: msg.amount.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl TryFrom<staking::v1beta1::MsgBeginRedelegate> for CosmosRawMsg {
    type Error = CosmosError;

    fn try_from(msg: staking::v1beta1::MsgBeginRedelegate) -> Result<Self, Self::Error> {
        let coin = msg
            .amount
            .ok_or_else(|| eyre::eyre!("Missing amount of MsgBeginRedelegate"))?;
        Ok(Self::Normal {
            msg: CosmosRawNormalMsg::StakingBeginRedelegate {
                delegator_address: msg.delegator_address,
                validator_src_address: msg.validator_src_address,
                validator_dst_address: msg.validator_dst_address,
                amount: coin.into(),
            },
        })
    }
}

impl TryFrom<staking::v1beta1::MsgDelegate> for CosmosRawMsg {
    type Error = CosmosError;

    fn try_from(msg: staking::v1beta1::MsgDelegate) -> Result<Self, Self::Error> {
        let coin = msg
            .amount
            .ok_or_else(|| eyre::eyre!("Missing amount of MsgDelegate"))?;
        Ok(Self::Normal {
            msg: CosmosRawNormalMsg::StakingDelegate {
                delegator_address: msg.delegator_address,
                validator_address: msg.validator_address,
                amount: coin.into(),
            },
        })
    }
}

impl TryFrom<staking::v1beta1::MsgUndelegate> for CosmosRawMsg {
    type Error = CosmosError;

    fn try_from(msg: staking::v1beta1::MsgUndelegate) -> Result<Self, Self::Error> {
        let coin = msg
            .amount
            .ok_or_else(|| eyre::eyre!("Missing amount of MsgUndelegate"))?;
        Ok(Self::Normal {
            msg: CosmosRawNormalMsg::StakingUndelegate {
                delegator_address: msg.delegator_address,
                validator_address: msg.validator_address,
                amount: coin.into(),
            },
        })
    }
}

impl From<distribution::v1beta1::MsgSetWithdrawAddress> for CosmosRawMsg {
    fn from(msg: distribution::v1beta1::MsgSetWithdrawAddress) -> Self {
        Self::Normal {
            msg: CosmosRawNormalMsg::DistributionSetWithdrawAddress {
                delegator_address: msg.delegator_address,
                withdraw_address: msg.withdraw_address,
            },
        }
    }
}

impl From<distribution::v1beta1::MsgWithdrawDelegatorReward> for CosmosRawMsg {
    fn from(msg: distribution::v1beta1::MsgWithdrawDelegatorReward) -> Self {
        Self::Normal {
            msg: CosmosRawNormalMsg::DistributionWithdrawDelegatorReward {
                delegator_address: msg.delegator_address,
                validator_address: msg.validator_address,
            },
        }
    }
}

impl TryFrom<MsgTransfer> for CosmosRawMsg {
    type Error = CosmosError;

    fn try_from(msg: MsgTransfer) -> Result<Self, Self::Error> {
        let coin = msg
            .token
            .ok_or_else(|| eyre::eyre!("Missing token of MsgTransfer"))?;
        Ok(Self::Normal {
            msg: CosmosRawNormalMsg::IbcTransfer {
                sender: msg.sender.to_string(),
                receiver: msg.receiver.to_string(),
                source_port: msg.source_port.to_string(),
                source_channel: msg.source_channel.to_string(),
                token: coin.into(),
                timeout_height: msg.timeout_height,
                timeout_timestamp: msg.timeout_timestamp.nanoseconds(),
            },
        })
    }
}

impl From<chainmain::nft::v1::MsgIssueDenom> for CosmosRawMsg {
    fn from(msg: chainmain::nft::v1::MsgIssueDenom) -> Self {
        Self::CryptoOrg {
            msg: CosmosRawCryptoOrgMsg::NftIssueDenom {
                id: msg.id,
                name: msg.name,
                schema: msg.schema,
                sender: msg.sender,
            },
        }
    }
}

impl From<chainmain::nft::v1::MsgMintNft> for CosmosRawMsg {
    fn from(msg: chainmain::nft::v1::MsgMintNft) -> Self {
        Self::CryptoOrg {
            msg: CosmosRawCryptoOrgMsg::NftMint {
                id: msg.id,
                denom_id: msg.denom_id,
                name: msg.name,
                uri: msg.uri,
                data: msg.data,
                sender: msg.sender,
                recipient: msg.recipient,
            },
        }
    }
}

impl From<chainmain::nft::v1::MsgEditNft> for CosmosRawMsg {
    fn from(msg: chainmain::nft::v1::MsgEditNft) -> Self {
        Self::CryptoOrg {
            msg: CosmosRawCryptoOrgMsg::NftEdit {
                id: msg.id,
                denom_id: msg.denom_id,
                name: msg.name,
                uri: msg.uri,
                data: msg.data,
                sender: msg.sender,
            },
        }
    }
}

impl From<chainmain::nft::v1::MsgTransferNft> for CosmosRawMsg {
    fn from(msg: chainmain::nft::v1::MsgTransferNft) -> Self {
        Self::CryptoOrg {
            msg: CosmosRawCryptoOrgMsg::NftTransfer {
                id: msg.id,
                denom_id: msg.denom_id,
                sender: msg.sender,
                recipient: msg.recipient,
            },
        }
    }
}

impl From<chainmain::nft::v1::MsgBurnNft> for CosmosRawMsg {
    fn from(msg: chainmain::nft::v1::MsgBurnNft) -> Self {
        Self::CryptoOrg {
            msg: CosmosRawCryptoOrgMsg::NftBurn {
                id: msg.id,
                denom_id: msg.denom_id,
                sender: msg.sender,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "@type")]
pub enum CosmosRawNormalMsg {
    /// MsgSend
    #[serde(rename = "/cosmos.bank.v1beta1.MsgSend")]
    BankSend {
        /// sender address in bech32
        from_address: String,
        /// recipient address in bech32
        to_address: String,
        /// amount to send
        amount: Vec<SingleCoin>,
    },
    /// MsgBeginRedelegate
    #[serde(rename = "/cosmos.staking.v1beta1.MsgBeginRedelegate")]
    StakingBeginRedelegate {
        /// delegator address in bech32
        delegator_address: String,
        /// source validator address in bech32
        validator_src_address: String,
        /// destination validator address in bech32
        validator_dst_address: String,
        /// amount to redelegate
        amount: SingleCoin,
    },
    /// MsgDelegate
    #[serde(rename = "/cosmos.staking.v1beta1.MsgDelegate")]
    StakingDelegate {
        /// delegator address in bech32
        delegator_address: String,
        /// validator address in bech32
        validator_address: String,
        /// amount to delegate
        amount: SingleCoin,
    },
    /// MsgUndelegate
    #[serde(rename = "/cosmos.staking.v1beta1.MsgUndelegate")]
    StakingUndelegate {
        /// delegator address in bech32
        delegator_address: String,
        /// validator address in bech32
        validator_address: String,
        /// amount to undelegate
        amount: SingleCoin,
    },
    /// MsgSetWithdrawAddress
    #[serde(rename = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress")]
    DistributionSetWithdrawAddress {
        /// delegator address in bech32
        delegator_address: String,
        /// withdraw address in bech32
        withdraw_address: String,
    },
    /// MsgWithdrawDelegatorReward
    #[serde(rename = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward")]
    DistributionWithdrawDelegatorReward {
        /// delegator address in bech32
        delegator_address: String,
        /// validator address in bech32
        validator_address: String,
    },
    /// MsgTransfer
    #[serde(rename = "/ibc.applications.transfer.v1.MsgTransfer")]
    IbcTransfer {
        /// the sender address
        sender: String,
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

impl CosmosRawNormalMsg {
    pub fn to_any(&self) -> eyre::Result<Any> {
        match self {
            Self::BankSend {
                from_address,
                to_address,
                amount,
            } => MsgSend {
                from_address: from_address.parse::<AccountId>()?,
                to_address: to_address.parse::<AccountId>()?,
                amount: amount
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            }
            .to_any(),
            Self::StakingBeginRedelegate {
                delegator_address,
                validator_src_address,
                validator_dst_address,
                amount,
            } => MsgBeginRedelegate {
                delegator_address: delegator_address.parse::<AccountId>()?,
                validator_src_address: validator_src_address.parse::<AccountId>()?,
                validator_dst_address: validator_dst_address.parse::<AccountId>()?,
                amount: amount.try_into()?,
            }
            .to_any(),
            Self::StakingDelegate {
                delegator_address,
                validator_address,
                amount,
            } => MsgDelegate {
                delegator_address: delegator_address.parse::<AccountId>()?,
                validator_address: validator_address.parse::<AccountId>()?,
                amount: amount.try_into()?,
            }
            .to_any(),
            Self::StakingUndelegate {
                delegator_address,
                validator_address,
                amount,
            } => MsgUndelegate {
                delegator_address: delegator_address.parse::<AccountId>()?,
                validator_address: validator_address.parse::<AccountId>()?,
                amount: amount.try_into()?,
            }
            .to_any(),
            Self::DistributionSetWithdrawAddress {
                delegator_address,
                withdraw_address,
            } => MsgSetWithdrawAddress {
                delegator_address: delegator_address.parse::<AccountId>()?,
                withdraw_address: withdraw_address.parse::<AccountId>()?,
            }
            .to_any(),
            Self::DistributionWithdrawDelegatorReward {
                delegator_address,
                validator_address,
            } => MsgWithdrawDelegatorReward {
                delegator_address: delegator_address.parse::<AccountId>()?,
                validator_address: validator_address.parse::<AccountId>()?,
            }
            .to_any(),
            Self::IbcTransfer {
                sender,
                receiver,
                source_port,
                source_channel,
                token,
                timeout_height,
                timeout_timestamp,
            } => {
                let any = MsgTransfer {
                    sender: Signer::new(sender),
                    receiver: Signer::new(receiver),
                    source_port: PortId::from_str(source_port)?,
                    source_channel: ChannelId::from_str(source_channel)?,
                    token: Some(token.try_into()?),
                    // TODO: timeout_height and timeout_timestamp cannot both be 0.
                    timeout_height: *timeout_height,
                    timeout_timestamp: Timestamp::from_nanoseconds(*timeout_timestamp)?,
                }
                .to_any();
                // FIXME:
                // ibc-proto used Google's Protobuf type definitions instead of
                // prost_types in `0.17`. But cosmrs still used prost_types. So
                // we need to convert manually.
                // Associate cosmos-rust issue:
                // https://github.com/cosmos/cosmos-rust/issues/185
                Ok(cosmrs::Any {
                    type_url: any.type_url,
                    value: any.value,
                })
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "@type")]
pub enum CosmosRawCryptoOrgMsg {
    /// MsgIssueDenom
    #[serde(rename = "/chainmain.nft.v1.MsgIssueDenom")]
    NftIssueDenom {
        /// The denomination ID of the NFT, necessary as multiple denominations are able to be represented on each chain.
        id: String,
        /// The denomination name of the NFT, necessary as multiple denominations are able to be represented on each chain.
        name: String,
        /// The account address of the user creating the denomination
        schema: String,
        /// the sender address
        sender: String,
    },
    /// MsgMintNft
    #[serde(rename = "/chainmain.nft.v1.MsgMintNFT")]
    NftMint {
        /// The unique ID of the NFT being minted
        id: String,
        /// The unique ID of the denomination
        denom_id: String,
        /// The name of the NFT being minted
        name: String,
        /// The URI pointing to a JSON object that contains subsequent tokenData information off-chain.
        uri: String,
        /// The data of the NFT
        data: String,
        /// the sender address
        sender: String,
        /// The recipient of the new NFT
        recipient: String,
    },
    /// MsgEditNft
    #[serde(rename = "/chainmain.nft.v1.MsgEditNFT")]
    NftEdit {
        /// The unique ID of the NFT being edited
        id: String,
        /// The unique ID of the denomination, necessary as multiple denominations are able to be represented on each chain.
        denom_id: String,
        /// The name of the NFT being edited
        name: String,
        /// The URI pointing to a JSON object that contains subsequent tokenData information off-chain.
        uri: String,
        /// The data of the NFT
        data: String,
        /// the sender address
        sender: String,
    },
    /// MsgTransferNft
    #[serde(rename = "/chainmain.nft.v1.MsgTransferNFT")]
    NftTransfer {
        /// The unique ID of the NFT being transferred
        id: String,
        /// The unique ID of the denomination, necessary as multiple denominations are able to be represented on each chain.
        denom_id: String,
        /// the sender address
        sender: String,
        /// The account address who will receive the NFT as a result of the transfer transaction.
        recipient: String,
    },
    /// MsgBurnNft
    #[serde(rename = "/chainmain.nft.v1.MsgBurnNFT")]
    NftBurn {
        /// The ID of the Token
        id: String,
        /// The Denom ID of the Token
        denom_id: String,
        /// the sender address
        sender: String,
    },
}

impl CosmosRawCryptoOrgMsg {
    pub fn to_any(&self) -> eyre::Result<Any> {
        match self {
            Self::NftIssueDenom {
                id,
                name,
                schema,
                sender,
            } => MsgIssueDenom {
                id: id.parse::<DenomId>()?,
                name: name.parse::<DenomName>()?,
                schema: schema.to_owned(),
                sender: sender.parse::<AccountId>()?,
            }
            .to_any(),
            Self::NftMint {
                id,
                denom_id,
                name,
                uri,
                data,
                sender,
                recipient,
            } => MsgMintNft {
                id: id.parse::<TokenId>()?,
                denom_id: denom_id.parse::<DenomId>()?,
                name: name.to_owned(),
                uri: uri.parse::<TokenUri>()?,
                data: data.to_owned(),
                sender: sender.parse::<AccountId>()?,
                recipient: recipient.parse::<AccountId>()?,
            }
            .to_any(),
            Self::NftEdit {
                id,
                denom_id,
                name,
                uri,
                data,
                sender,
            } => MsgEditNft {
                id: id.parse::<TokenId>()?,
                denom_id: denom_id.parse::<DenomId>()?,
                name: name.to_owned(),
                uri: uri.parse::<TokenUri>()?,
                data: data.to_owned(),
                sender: sender.parse::<AccountId>()?,
            }
            .to_any(),
            Self::NftTransfer {
                id,
                denom_id,
                sender,
                recipient,
            } => MsgTransferNft {
                id: id.parse::<TokenId>()?,
                denom_id: denom_id.parse::<DenomId>()?,
                sender: sender.parse::<AccountId>()?,
                recipient: recipient.parse::<AccountId>()?,
            }
            .to_any(),
            Self::NftBurn {
                id,
                denom_id,
                sender,
            } => MsgBurnNft {
                id: id.parse::<TokenId>()?,
                denom_id: denom_id.parse::<DenomId>()?,
                sender: sender.parse::<AccountId>()?,
            }
            .to_any(),
        }
    }
}
