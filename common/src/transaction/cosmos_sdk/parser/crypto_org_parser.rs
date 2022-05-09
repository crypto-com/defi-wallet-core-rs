use crate::proto::chainmain::nft::v1::{
    MsgBurnNft, MsgEditNft, MsgIssueDenom, MsgMintNft, MsgTransferNft,
};
use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::structs::{CosmosRawMsg, CosmosTxBody};
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;
use cosmrs::tx::MsgProto;
use eyre::WrapErr;
use prost::Message;

/// Cosmos parser for `crypto.org` chain
pub(crate) struct CryptoOrgParser {
    base: BaseParser,
}

impl CosmosParser for CryptoOrgParser {
    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError> {
        self.base.transform_tx_body(tx_body)?;
        tx_body.messages = tx_body
            .messages
            .iter()
            .map(transform_msg)
            .collect::<Result<_, _>>()?;
        Ok(())
    }
}

impl CryptoOrgParser {
    pub fn new() -> Self {
        Self {
            base: BaseParser {},
        }
    }
}

// Transform `CosmosRawMsg::Any` messages to special `crypto.com` ones.
fn transform_msg(msg: &CosmosRawMsg) -> Result<CosmosRawMsg, CosmosError> {
    if let CosmosRawMsg::Any { type_url, value } = msg {
        Ok(match type_url.as_str() {
            MsgBurnNft::TYPE_URL => MsgBurnNft::decode(value.as_slice())
                .wrap_err("Failed to decode MsgBurnNft from Protobuf")?
                .into(),
            MsgEditNft::TYPE_URL => MsgEditNft::decode(value.as_slice())
                .wrap_err("Failed to decode MsgEditNft from Protobuf")?
                .into(),
            MsgIssueDenom::TYPE_URL => MsgIssueDenom::decode(value.as_slice())
                .wrap_err("Failed to decode MsgIssueDenom from Protobuf")?
                .into(),
            MsgMintNft::TYPE_URL => MsgMintNft::decode(value.as_slice())
                .wrap_err("Failed to decode MsgMintNft from Protobuf")?
                .into(),
            MsgTransferNft::TYPE_URL => MsgTransferNft::decode(value.as_slice())
                .wrap_err("Failed to decode MsgTransferNft from Protobuf")?
                .into(),
            _ => msg.clone(),
        })
    } else {
        Ok(msg.clone())
    }
}

#[cfg(test)]
mod cosmos_crypto_org_parsing_tests {
    use super::*;

    #[test]
    fn test_proto_body_parsing() {
        let body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

        let parser = CosmosParser::new();
        let body = parser.parse_proto_body(body_bytes).unwrap();
    }
}
