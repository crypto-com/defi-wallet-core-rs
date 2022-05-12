use crate::proto::chainmain::nft::v1::{
    MsgBurnNft, MsgEditNft, MsgIssueDenom, MsgMintNft, MsgTransferNft,
};
use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::structs::{
    CosmosRawCryptoOrgMsg, CosmosRawMsg, CosmosRawNormalMsg, CosmosTxBody,
};
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
    fn parse_amino_json_msg(&self, json_string: &str) -> Result<CosmosRawMsg, CosmosError> {
        Ok(serde_json::from_str::<CosmosRawNormalMsg>(json_string)
            .map(|msg| CosmosRawMsg::Normal { msg })
            .or_else(|_| {
                serde_json::from_str::<CosmosRawCryptoOrgMsg>(json_string)
                    .map(|msg| CosmosRawMsg::CryptoOrg { msg })
            })
            .wrap_err("Failed to decode CosmosRawMsg from Amino JSON")?)
    }

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

#[cfg(test)]
mod cosmos_crypto_org_parsing_tests {
    use super::*;
    use crate::transaction::cosmos_sdk::parser::structs::CosmosRawMsg;

    #[test]
    fn test_amino_json_msg_parsing() {
        let json_msg = "{\"@type\":\"/chainmain.nft.v1.MsgMintNFT\",\"id\":\"test_token_id\",\"denom_id\":\"test_denom_id\",\"name\":\"\",\"uri\":\"test_uri\",\"data\":\"\",\"sender\":\"cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6\",\"recipient\":\"cosmos1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu\"}";

        let parser = CryptoOrgParser {
            base: BaseParser {},
        };
        let msg = parser.parse_amino_json_msg(json_msg).unwrap();

        assert_eq!(
            msg,
            CosmosRawMsg::CryptoOrg {
                msg: CosmosRawCryptoOrgMsg::NftMint {
                    id: "test_token_id".to_string(),
                    denom_id: "test_denom_id".to_string(),
                    name: "".to_string(),
                    uri: "test_uri".to_string(),
                    data: "".to_string(),
                    sender: "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
                    recipient: "cosmos1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu".to_string(),
                },
            },
        );
    }
}
