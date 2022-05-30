// ! Luna Classic wasm module support

use crate::{proto, AccountId, Coin, ErrorReport, Msg, Result};

/// MsgExecuteContract submits the given message data to a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,

    /// ExecuteMsg json encoded message to be passed to the contract
    pub execute_msg: Vec<u8>,

    /// Coins that are transferred to the contract on execution
    pub coins: Vec<Coin>,
}

impl Msg for MsgExecuteContract {
    type Proto = proto::luna_classic::wasm::v1beta1::MsgExecuteContract;
}

impl TryFrom<proto::luna_classic::wasm::v1beta1::MsgExecuteContract> for MsgExecuteContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::luna_classic::wasm::v1beta1::MsgExecuteContract,
    ) -> Result<MsgExecuteContract> {
        Ok(MsgExecuteContract {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
            execute_msg: proto.execute_msg.into_iter().map(Into::into).collect(),
            coins: proto
                .coins
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgExecuteContract> for proto::luna_classic::wasm::v1beta1::MsgExecuteContract {
    fn from(msg: MsgExecuteContract) -> proto::luna_classic::wasm::v1beta1::MsgExecuteContract {
        proto::luna_classic::wasm::v1beta1::MsgExecuteContract {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
            execute_msg: msg.execute_msg,
            coins: msg.coins.iter().map(Into::into).collect(),
        }
    }
}
