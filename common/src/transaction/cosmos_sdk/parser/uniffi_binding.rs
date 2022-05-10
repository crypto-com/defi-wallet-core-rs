#![cfg(feature = "uniffi-binding")]

use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::crypto_org_parser::CryptoOrgParser;
use crate::transaction::cosmos_sdk::parser::structs::{CosmosAuthInfo, CosmosTxBody};
use crate::transaction::cosmos_sdk::parser::terra_parser::TerraParser;
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;

pub struct CosmosParserWrapper {
    inner: Box<dyn CosmosParser + Send + Sync>,
}

impl CosmosParserWrapper {
    /// Create a Cosmos parser for `crypto.org` chain.
    pub fn new_crypto_org_parser() -> Self {
        Self {
            inner: Box::new(CryptoOrgParser {
                base: BaseParser {},
            }),
        }
    }

    /// Create a Cosmos parser for `Terra` chain.
    pub fn new_terra_parser() -> Self {
        Self {
            inner: Box::new(TerraParser {
                base: BaseParser {},
            }),
        }
    }

    /// Parse `CosmosAuthInfo` from hex data of Protobuf.
    pub fn parse_proto_auto_info(&self, hex_string: &str) -> Result<CosmosAuthInfo, CosmosError> {
        self.inner.parse_proto_auto_info(hex_string)
    }

    /// Parse `CosmosTxBody` from hex data of Protobuf.
    pub fn parse_proto_tx_body(&self, hex_string: &str) -> Result<CosmosTxBody, CosmosError> {
        self.inner.parse_proto_tx_body(hex_string)
    }
}
