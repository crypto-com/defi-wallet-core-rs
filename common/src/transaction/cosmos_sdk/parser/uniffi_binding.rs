#![cfg(feature = "uniffi-binding")]

use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
use crate::transaction::cosmos_sdk::parser::crypto_org_parser::CryptoOrgParser;
use crate::transaction::cosmos_sdk::parser::structs::{
    CosmosAuthInfo, CosmosFee, CosmosRawMsg, CosmosTxBody,
};
use crate::transaction::cosmos_sdk::parser::luna_classic_parser::LunaClassicParser;
use crate::transaction::cosmos_sdk::parser::CosmosParser;
use crate::transaction::cosmos_sdk::CosmosError;

pub struct CosmosParserWrapper {
    inner: Box<dyn CosmosParser + Send + Sync>,
}

impl CosmosParserWrapper {
    /// Create a base parser for decoding standard Cosmos messages.
    pub fn new_base_parser() -> Self {
        Self {
            inner: Box::new(BaseParser {}),
        }
    }

    /// Create a Cosmos parser for `crypto.org` chain.
    pub fn new_crypto_org_parser() -> Self {
        Self {
            inner: Box::new(CryptoOrgParser {
                base: BaseParser {},
            }),
        }
    }

    /// Create a Cosmos parser for `LunaClassic` chain.
    pub fn new_luna_classic_parser() -> Self {
        Self {
            inner: Box::new(LunaClassicParser {
                base: BaseParser {},
            }),
        }
    }

    /// Parse `CosmosFee` from data of proto JSON mapping.
    pub fn parse_proto_json_fee(&self, json_string: &str) -> Result<CosmosFee, CosmosError> {
        self.inner.parse_proto_json_fee(json_string)
    }

    /// Parse `CosmosRawMsg` from data of proto JSON mapping.
    pub fn parse_proto_json_msg(&self, json_string: &str) -> Result<CosmosRawMsg, CosmosError> {
        self.inner.parse_proto_json_msg(json_string)
    }

    /// Parse `CosmosAuthInfo` from hex data of Protobuf.
    pub fn parse_protobuf_auto_info(
        &self,
        hex_string: &str,
    ) -> Result<CosmosAuthInfo, CosmosError> {
        self.inner.parse_protobuf_auto_info(hex_string)
    }

    /// Parse `CosmosTxBody` from hex data of Protobuf.
    pub fn parse_protobuf_tx_body(&self, hex_string: &str) -> Result<CosmosTxBody, CosmosError> {
        self.inner.parse_protobuf_tx_body(hex_string)
    }
}
