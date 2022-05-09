use crate::transaction::cosmos_sdk::parser::structs::CosmosHeight;
use crate::transaction::cosmos_sdk::{CosmosError, SingleCoin};
use cosmrs::Any;
use serde::{Deserialize, Serialize};

/// Cosmos raw message that is parsed from Protobuf or JSON.
/// FIXME:
/// Since `CosmosSDKMsg` is constructed by fields and has no `sender_address` which is a wallet
/// address. `CosmosRawMsg` is parsed directly from Protobuf or JSON, it should have the all fields
/// of original message.
#[derive(Clone, Deserialize, Serialize)]
pub enum CosmosRawMsg {
    /// Normal message
    Normal { msg: CosmosRawNormalMsg },
    /// `crypto.org` special message
    CryptoOrg { msg: CosmosRawCryptoOrgMsg },
    /// `Terra` special message
    Terra { msg: CosmosRawTerraMsg },
    /// Any message
    /// It is only used for messages which has not been supported.
    Any { type_url: String, value: Vec<u8> },
}

impl CosmosRawMsg {
    pub fn to_any(&self) -> Result<Any, CosmosError> {
        todo!()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum CosmosRawNormalMsg {
    /// MsgSend
    BankSend {
        /// sender address in bech32
        from_address: String,
        /// recipient address in bech32
        to_address: String,
        /// amount to send
        amount: SingleCoin,
    },
    /// MsgBeginRedelegate
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
    StakingDelegate {
        /// delegator address in bech32
        delegator_address: String,
        /// validator address in bech32
        validator_address: String,
        /// amount to delegate
        amount: SingleCoin,
    },
    /// MsgUndelegate
    StakingUndelegate {
        /// delegator address in bech32
        delegator_address: String,
        /// validator address in bech32
        validator_address: String,
        /// amount to undelegate
        amount: SingleCoin,
    },
    /// MsgSetWithdrawAddress
    DistributionSetWithdrawAddress {
        /// delegator address in bech32
        delegator_address: String,
        /// withdraw address in bech32
        withdraw_address: String,
    },
    /// MsgWithdrawDelegatorReward
    DistributionWithdrawDelegatorReward {
        /// delegator address in bech32
        delegator_address: String,
        /// validator address in bech32
        validator_address: String,
    },
    /// MsgTransfer
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
        timeout_height: CosmosHeight,
        /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
        /// The timeout is disabled when set to 0.
        timeout_timestamp: u64,
    },
}

#[derive(Clone, Deserialize, Serialize)]
pub enum CosmosRawCryptoOrgMsg {
    /// MsgIssueDenom
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
    NftBurn {
        /// The ID of the Token
        id: String,
        /// The Denom ID of the Token
        denom_id: String,
        /// the sender address
        sender: String,
    },
}

#[derive(Clone, Deserialize, Serialize)]
pub enum CosmosRawTerraMsg {
    // TODO: Add `Terra` special messages here. Wait for proto integration.
}
