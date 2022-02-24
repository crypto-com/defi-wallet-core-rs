// ! IBC module support

use crate::{ErrorReport, Result};
use cosmos_sdk_proto::ibc::applications::transfer;
use cosmos_sdk_proto::ibc::core::client;
use cosmrs::{AccountId, Any, Coin};

/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between
/// ICS20 enabled chains. See ICS Spec here:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgTransfer {
    /// the sender address
    pub sender: AccountId,
    /// the recipient address on the destination chain
    pub receiver: String,
    /// the port on which the packet will be sent
    pub source_port: String,
    /// the channel by which the packet will be sent
    pub source_channel: String,
    /// the tokens to be transferred
    pub token: Option<Coin>,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    pub timeout_height: Option<Height>,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    pub timeout_timestamp: u64,
}

impl MsgTransfer {
    const TYPE_URL: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";

    /// Serialize this message proto as [`Any`].
    pub fn to_any(&self) -> Result<Any> {
        let mut bytes = Vec::new();
        let proto: transfer::v1::MsgTransfer = self.into();
        prost::Message::encode(&proto, &mut bytes)?;

        Ok(Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
    }
}

impl TryFrom<transfer::v1::MsgTransfer> for MsgTransfer {
    type Error = ErrorReport;

    fn try_from(proto: transfer::v1::MsgTransfer) -> Result<MsgTransfer> {
        MsgTransfer::try_from(&proto)
    }
}

impl TryFrom<&transfer::v1::MsgTransfer> for MsgTransfer {
    type Error = ErrorReport;

    fn try_from(proto: &transfer::v1::MsgTransfer) -> Result<MsgTransfer> {
        Ok(MsgTransfer {
            sender: proto.sender.parse()?,
            receiver: proto.receiver.to_owned(),
            source_port: proto.source_port.to_owned(),
            source_channel: proto.source_channel.to_owned(),
            token: proto.token.as_ref().map(TryFrom::try_from).transpose()?,
            timeout_height: proto.timeout_height.as_ref().map(Into::into),
            timeout_timestamp: proto.timeout_timestamp,
        })
    }
}

impl From<MsgTransfer> for transfer::v1::MsgTransfer {
    fn from(msg: MsgTransfer) -> transfer::v1::MsgTransfer {
        transfer::v1::MsgTransfer::from(&msg)
    }
}

impl From<&MsgTransfer> for transfer::v1::MsgTransfer {
    fn from(msg: &MsgTransfer) -> transfer::v1::MsgTransfer {
        transfer::v1::MsgTransfer {
            sender: msg.sender.to_string(),
            receiver: msg.receiver.to_owned(),
            source_port: msg.source_port.to_owned(),
            source_channel: msg.source_channel.to_owned(),
            token: msg.token.as_ref().map(Into::into),
            timeout_height: msg.timeout_height.as_ref().map(Into::into),
            timeout_timestamp: msg.timeout_timestamp,
        }
    }
}

/// Height is a monotonically increasing data type that can be compared against
/// another Height for the purposes of updating and freezing clients.
///
/// Normally the RevisionHeight is incremented at each height while keeping
/// RevisionNumber the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the RevisionNumber is incremented so that
/// height continues to be monitonically increasing even as the RevisionHeight
/// gets reset.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Height {
    /// the revision that the client is currently on
    pub revision_number: u64,
    /// the height within the given revision
    pub revision_height: u64,
}

impl From<client::v1::Height> for Height {
    fn from(proto: client::v1::Height) -> Height {
        Height::from(&proto)
    }
}

impl From<&client::v1::Height> for Height {
    fn from(proto: &client::v1::Height) -> Height {
        Height {
            revision_number: proto.revision_number,
            revision_height: proto.revision_height,
        }
    }
}

impl From<Height> for client::v1::Height {
    fn from(height: Height) -> client::v1::Height {
        client::v1::Height::from(&height)
    }
}

impl From<&Height> for client::v1::Height {
    fn from(height: &Height) -> client::v1::Height {
        client::v1::Height {
            revision_number: height.revision_number,
            revision_height: height.revision_height,
        }
    }
}
