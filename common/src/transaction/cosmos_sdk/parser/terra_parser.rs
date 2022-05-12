use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::structs::{CosmosRawMsg, CosmosTxBody};
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;
use eyre::WrapErr;

/// Cosmos parser for `Terra` chain
pub(crate) struct TerraParser {
    pub base: BaseParser,
}

impl CosmosParser for TerraParser {
    fn parse_amino_json_msg(&self, json_string: &str) -> Result<CosmosRawMsg, CosmosError> {
        // TODO: Process `Terra` special messages.
        Ok(CosmosRawMsg::Normal {
            msg: serde_json::from_str(json_string)
                .wrap_err("Failed to parse to Cosmos message from an Amino JSON string")?,
        })
    }

    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError> {
        // TODO: Process `Terra` special messages.
        self.base.transform_tx_body(tx_body)
    }
}
