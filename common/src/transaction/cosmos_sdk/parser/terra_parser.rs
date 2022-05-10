use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::structs::CosmosTxBody;
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;

/// Cosmos parser for `Terra` chain
pub(crate) struct TerraParser {
    base: BaseParser,
}

impl CosmosParser for TerraParser {
    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError> {
        // TODO: Process `Terra` special messages.
        self.base.transform_tx_body(tx_body)
    }
}

impl TerraParser {
    pub fn new() -> Self {
        Self {
            base: BaseParser {},
        }
    }
}
