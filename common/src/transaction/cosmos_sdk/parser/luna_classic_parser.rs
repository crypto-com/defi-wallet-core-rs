use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::structs::{CosmosRawMsg, CosmosTxBody};
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;
use eyre::WrapErr;

/// Cosmos parser for `LunaClassic` chain
pub(crate) struct LunaClassicParser {
    pub base: BaseParser,
}

impl CosmosParser for LunaClassicParser {
    fn parse_proto_json_msg(&self, json_string: &str) -> Result<CosmosRawMsg, CosmosError> {
        // TODO: Process `LunaClassic` special messages.
        Ok(CosmosRawMsg::Normal {
            msg: serde_json::from_str(json_string)
                .wrap_err("Failed to decode CosmosRawMsg from proto JSON mapping")?,
        })
    }

    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError> {
        // TODO: Process `LunaClassic` special messages.
        self.base.transform_tx_body(tx_body)
    }
}
