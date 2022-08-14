use crate::transaction::cosmos_sdk::parser::structs::{CosmosRawMsg, CosmosTxBody};
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::distribution::v1beta1::{
    MsgSetWithdrawAddress, MsgWithdrawDelegatorReward,
};
use cosmos_sdk_proto::cosmos::staking::v1beta1::{MsgBeginRedelegate, MsgDelegate, MsgUndelegate};
use cosmos_sdk_proto::traits::{Message, TypeUrl};
use eyre::WrapErr;
use ibc::applications::transfer::msgs::transfer;
use tendermint_proto::Protobuf;

/// Base parser for standard Cosmos messages
pub struct BaseParser;

impl CosmosParser for BaseParser {
    fn parse_proto_json_msg(&self, json_string: &str) -> Result<CosmosRawMsg, CosmosError> {
        Ok(CosmosRawMsg::Normal {
            msg: serde_json::from_str(json_string)
                .wrap_err("Failed to decode CosmosRawMsg from proto JSON mapping")?,
        })
    }

    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError> {
        tx_body.messages = tx_body
            .messages
            .iter()
            .map(transform_msg)
            .collect::<Result<_, _>>()?;
        Ok(())
    }
}

// Transform `CosmosRawMsg::Any` messages to standard Cosmos ones.
fn transform_msg(msg: &CosmosRawMsg) -> Result<CosmosRawMsg, CosmosError> {
    if let CosmosRawMsg::Any { type_url, value } = msg {
        Ok(match type_url.as_str() {
            MsgSend::TYPE_URL => MsgSend::decode(value.as_slice())
                .wrap_err("Failed to decode MsgSend from Protobuf")?
                .into(),
            MsgBeginRedelegate::TYPE_URL => MsgBeginRedelegate::decode(value.as_slice())
                .wrap_err("Failed to decode MsgBeginRedelegate from Protobuf")?
                .try_into()?,
            MsgDelegate::TYPE_URL => MsgDelegate::decode(value.as_slice())
                .wrap_err("Failed to decode MsgDelegate from Protobuf")?
                .try_into()?,
            MsgUndelegate::TYPE_URL => MsgUndelegate::decode(value.as_slice())
                .wrap_err("Failed to decode MsgUndelegate from Protobuf")?
                .try_into()?,
            MsgSetWithdrawAddress::TYPE_URL => MsgSetWithdrawAddress::decode(value.as_slice())
                .wrap_err("Failed to decode MsgSetWithdrawAddress from Protobuf")?
                .into(),
            MsgWithdrawDelegatorReward::TYPE_URL => {
                MsgWithdrawDelegatorReward::decode(value.as_slice())
                    .wrap_err("Failed to decode MsgWithdrawDelegatorReward from Protobuf")?
                    .into()
            }
            transfer::TYPE_URL => transfer::MsgTransfer::decode(value.as_slice())
                .wrap_err("Failed to decode MsgTransfer from Protobuf")?
                .try_into()?,
            _ => msg.clone(),
        })
    } else {
        Ok(msg.clone())
    }
}

#[cfg(test)]
mod cosmos_base_parsing_tests {
    use super::*;
    use crate::transaction::cosmos_sdk::parser::structs::{CosmosRawMsg, CosmosRawNormalMsg};
    use crate::transaction::cosmos_sdk::SingleCoin;

    #[test]
    fn test_proto_json_msg_parsing() {
        let json_msg = "{\"@type\":\"/cosmos.bank.v1beta1.MsgSend\",\"amount\":[{\"amount\":\"1234567\",\"denom\":\"ucosm\"}],\"from_address\":\"cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6\",\"to_address\":\"cosmos1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu\"}";

        let parser = BaseParser {};
        let msg = parser.parse_proto_json_msg(json_msg).unwrap();

        assert_eq!(
            msg,
            CosmosRawMsg::Normal {
                msg: CosmosRawNormalMsg::BankSend {
                    from_address: "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
                    to_address: "cosmos1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu".to_string(),
                    amount: vec![SingleCoin::Other {
                        amount: "1234567".to_string(),
                        denom: "ucosm".to_string()
                    }],
                },
            },
        );
    }

    #[test]
    fn test_protobuf_tx_body_parsing() {
        let tx_body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

        let parser = BaseParser {};
        let tx_body = parser.parse_protobuf_tx_body(tx_body_bytes).unwrap();

        assert_eq!(
            tx_body,
            CosmosTxBody {
                messages: vec![CosmosRawMsg::Normal {
                    msg: CosmosRawNormalMsg::BankSend {
                        from_address: "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
                        to_address: "cosmos1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu".to_string(),
                        amount: vec![SingleCoin::Other {
                            amount: "1234567".to_string(),
                            denom: "ucosm".to_string()
                        }]
                    }
                }],
                memo: "".to_string(),
                timeout_height: 0,
                extension_options: vec![],
                non_critical_extension_options: vec![],
            }
        );
    }
}
