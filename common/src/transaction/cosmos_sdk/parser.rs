// Cosmos parser is used to deserialize Protobuf or Amino JSON to specified structs. The parsed
// instances could be encoded to a JSON string for display, and `CosmosRawMsg`s could be used to
// build a new transaction.

use crate::transaction::cosmos_sdk::CosmosError;
use crate::utils::hex_decode;
use cosmrs::tx::{AuthInfo, Body};
use eyre::WrapErr;
use prost::Message;

mod base_parser;
mod crypto_org_parser;
mod structs;
mod terra_parser;
mod uniffi_binding;

pub use structs::*;
#[cfg(feature = "uniffi-binding")]
pub use uniffi_binding::*;

/// Cosmos parser trait
pub trait CosmosParser {
    /// Parse `CosmosAuthInfo` from hex data of Protobuf.
    fn parse_proto_auto_info(&self, hex_string: &str) -> Result<CosmosAuthInfo, CosmosError> {
        let bytes = hex_decode(hex_string).wrap_err("Failed to decode hex string")?;
        let auth_info = AuthInfo::try_from(
            cosmos_sdk_proto::cosmos::tx::v1beta1::AuthInfo::decode(bytes.as_slice())
                .wrap_err("Failed to decode AuthInfo from Protobuf")?,
        )?;
        auth_info.try_into()
    }

    /// Parse `CosmosTxBody` from hex data of Protobuf.
    fn parse_proto_tx_body(&self, hex_string: &str) -> Result<CosmosTxBody, CosmosError> {
        let bytes = hex_decode(hex_string).wrap_err("Failed to decode hex string")?;
        let mut tx_body = Body::try_from(
            cosmos_sdk_proto::cosmos::tx::v1beta1::TxBody::decode(bytes.as_slice())
                .wrap_err("Failed to decode TxBody from Protobuf")?,
        )?
        .into();

        self.transform_tx_body(&mut tx_body)?;
        Ok(tx_body)
    }

    /// Transform `CosmosTxBody` for specified chain.
    /// This trait function must be implemented by sub-struct. The field `messages` has been
    /// initialized to type `CosmosRawMsg::Any` which should be transformed to detailed messages of
    /// specified chain.
    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError>;
}

#[cfg(test)]
mod cosmos_parsing_tests {
    use super::*;

    #[test]
    fn test_proto_auth_info_parsing() {
        let auth_info_bytes = "0a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c";

        let parser = CosmosParser::new();
        let auto_info = parser.parse_proto_auto_info(auth_info_bytes).unwrap();
    }
}
