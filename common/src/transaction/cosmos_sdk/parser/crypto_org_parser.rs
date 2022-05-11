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
    pub base: BaseParser,
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
