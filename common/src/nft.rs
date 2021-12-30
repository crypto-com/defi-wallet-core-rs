// ! NFT module support

use crate::{proto, AccountId, ErrorReport, Msg, Result};

// MsgIssueDenom defines an SDK message for creating a new denom.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgIssueDenom {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub sender: AccountId,
}

impl Msg for MsgIssueDenom {
    type Proto = proto::chainmain::nft::v1::MsgIssueDenom;
}

impl TryFrom<proto::chainmain::nft::v1::MsgIssueDenom> for MsgIssueDenom {
    type Error = ErrorReport;

    fn try_from(proto: proto::chainmain::nft::v1::MsgIssueDenom) -> Result<MsgIssueDenom> {
        MsgIssueDenom::try_from(&proto)
    }
}

impl TryFrom<&proto::chainmain::nft::v1::MsgIssueDenom> for MsgIssueDenom {
    type Error = ErrorReport;

    fn try_from(proto: &proto::chainmain::nft::v1::MsgIssueDenom) -> Result<MsgIssueDenom> {
        Ok(MsgIssueDenom {
            id: proto.id.parse()?,
            name: proto.name.parse()?,
            schema: proto.schema.parse()?,
            sender: proto.sender.parse()?,
        })
    }
}

impl From<MsgIssueDenom> for proto::chainmain::nft::v1::MsgIssueDenom {
    fn from(denom: MsgIssueDenom) -> proto::chainmain::nft::v1::MsgIssueDenom {
        proto::chainmain::nft::v1::MsgIssueDenom::from(&denom)
    }
}

impl From<&MsgIssueDenom> for proto::chainmain::nft::v1::MsgIssueDenom {
    fn from(msg: &MsgIssueDenom) -> proto::chainmain::nft::v1::MsgIssueDenom {
        proto::chainmain::nft::v1::MsgIssueDenom {
            id: msg.id.to_string(),
            name: msg.name.to_string(),
            schema: msg.schema.to_string(),
            sender: msg.sender.to_string(),
        }
    }
}

// MsgMintNFT defines an SDK message for creating a new NFT.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMintNFT {
    pub id: String,
    pub denom_id: String,
    pub name: String,
    pub uri: String,
    pub data: String,
    pub sender: AccountId,
    pub recipient: AccountId,
}

// MsgEditNFT defines an SDK message for editing a nft.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgEditNFT {
    pub id: String,
    pub denom_id: String,
    pub name: String,
    pub uri: String,
    pub data: String,
    pub sender: AccountId,
}

// MsgTransferNFT defines an SDK message for transferring an NFT to recipient.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgTransferNFT {
    pub id: String,
    pub denom_id: String,
    pub sender: AccountId,
    pub recipient: AccountId,
}

// MsgBurnNFT defines an SDK message for burning a NFT.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgBurnNFT {
    pub id: String,
    pub denom_id: String,
    pub sender: AccountId,
}
