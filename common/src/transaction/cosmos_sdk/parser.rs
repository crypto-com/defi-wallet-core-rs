// Cosmos parser is used to deserialize Protobuf or proto JSON mapping to specified structs. The
// parsed instances could be encoded to a JSON string for display, and `CosmosRawMsg`s could be
// used to build a new transaction.

use crate::transaction::cosmos_sdk::CosmosError;
use crate::utils::hex_decode;
use cosmrs::tx::{AuthInfo, Body};
use eyre::WrapErr;
use prost::Message;

mod base_parser;
mod crypto_org_parser;
mod luna_classic_parser;
mod structs;
mod uniffi_binding;

pub use structs::*;
#[cfg(feature = "uniffi-binding")]
pub use uniffi_binding::*;

/// Cosmos parser trait
pub trait CosmosParser {
    /// Parse `CosmosFee` from data of proto JSON mapping.
    fn parse_proto_json_fee(&self, json_string: &str) -> Result<CosmosFee, CosmosError> {
        Ok(serde_json::from_str(json_string)
            .wrap_err("Failed to decode CosmosFee from proto JSON mapping")?)
    }

    /// Parse `CosmosAuthInfo` from hex data of Protobuf.
    fn parse_protobuf_auto_info(&self, hex_string: &str) -> Result<CosmosAuthInfo, CosmosError> {
        let bytes = hex_decode(hex_string).wrap_err("Failed to decode hex string")?;
        let auth_info = AuthInfo::try_from(
            cosmos_sdk_proto::cosmos::tx::v1beta1::AuthInfo::decode(bytes.as_slice())
                .wrap_err("Failed to decode AuthInfo from Protobuf")?,
        )?;
        auth_info.try_into()
    }

    /// Parse `CosmosTxBody` from hex data of Protobuf.
    fn parse_protobuf_tx_body(&self, hex_string: &str) -> Result<CosmosTxBody, CosmosError> {
        let bytes = hex_decode(hex_string).wrap_err("Failed to decode hex string")?;
        let mut tx_body = Body::try_from(
            cosmos_sdk_proto::cosmos::tx::v1beta1::TxBody::decode(bytes.as_slice())
                .wrap_err("Failed to decode TxBody from Protobuf")?,
        )?
        .into();

        self.transform_tx_body(&mut tx_body)?;
        Ok(tx_body)
    }

    /// Parse `CosmosRawMsg` from data of proto JSON mapping.
    fn parse_proto_json_msg(&self, json_string: &str) -> Result<CosmosRawMsg, CosmosError>;

    /// Transform `CosmosTxBody` for specified chain.
    /// This trait function must be implemented by sub-struct. The field `messages` has been
    /// initialized to type `CosmosRawMsg::Any` which should be transformed to detailed messages of
    /// specified chain.
    fn transform_tx_body(&self, tx_body: &mut CosmosTxBody) -> Result<(), CosmosError>;
}

#[cfg(test)]
mod cosmos_parsing_tests {
    use super::*;
    use crate::transaction::cosmos_sdk::parser::base_parser::BaseParser;
    use crate::transaction::cosmos_sdk::SingleCoin;

    #[test]
    fn test_proto_json_fee_parsing() {
        let json_fee = "{\"amount\":[{\"amount\":\"27743\",\"denom\":\"uusd\"}],\"gas_limit\":\"184953\",\"payer\":\"\"}";

        let parser = BaseParser {};
        let fee = parser.parse_proto_json_fee(json_fee).unwrap();

        assert_eq!(
            fee,
            CosmosFee {
                amount: vec![SingleCoin::Other {
                    amount: "27743".to_string(),
                    denom: "uusd".to_string()
                }],
                gas_limit: 184953,
                payer: Some("".to_string()),
                granter: None,
            },
        );
    }

    #[test]
    fn test_protobuf_auth_info_parsing() {
        let auth_info_bytes = "0a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c";

        let parser = BaseParser {};
        let auth_info = parser.parse_protobuf_auto_info(auth_info_bytes).unwrap();

        assert_eq!(
            auth_info,
            CosmosAuthInfo {
                fee: CosmosFee {
                    amount: vec![SingleCoin::Other {
                        amount: "2000".to_string(),
                        denom: "ucosm".to_string()
                    }],
                    gas_limit: 200000,
                    payer: None,
                    granter: None,
                },
                signer_infos: vec![CosmosSignerInfo {
                    public_key: Some(CosmosSignerPublicKey::Any {
                        key: CosmosAny {
                            type_url: "".to_string(),
                            value: "".to_string()
                        }
                    }),
                    mode_info: CosmosModeInfo::Single {
                        mode: "Direct".to_string()
                    },
                    sequence: 1
                }]
            }
        );
    }
}
