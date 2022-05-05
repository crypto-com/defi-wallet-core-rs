/// Cosmos parser is used to deserialize Protobuf or JSON to specified structs. The parsed
/// instances could be encoded to a JSON string for display, and `CosmosSDKMsg`s could be used to
/// build a new transaction.
/// FIXME: It seems that these structs are only used for Cosmos parsing for now. They could be
/// moved to `cosmos_sdk.rs` if reusable.
use crate::transaction::cosmos_sdk::{CosmosError, CosmosSDKMsg, SingleCoin};
use crate::utils::hex_decode;
use prost::Message;
use serde::{Deserialize, Serialize};

/// Any contains arbitrary data along with a URL that describes the data type.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosAny {
    /// URL of data type
    pub type_url: String,
    /// Base64 encoded data
    pub value: String,
}

/// AuthInfo describes the fee and signer modes that are used to sign a transaction.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosAuthInfo {
    /// Signing modes for the required signers
    pub signer_infos: Vec<CosmosSignerInfo>,
    /// Fee and gas limit
    pub fee: CosmosFee,
}

/// Body of a transaction that all signers sign over.
#[derive(Deserialize, Serialize)]
pub struct CosmosBody {
    /// Message list
    pub messages: Vec<CosmosSDKMsg>,
    /// Memo
    pub memo: String,
    /// Timeout
    pub timeout_height: u64,
    /// Extension options
    pub extension_options: Vec<CosmosAny>,
    /// Non critical extension options
    pub non_critical_extension_options: Vec<CosmosAny>,
}

/// Fee includes the amount of coins paid in fees and the maximum gas to be used by the transaction.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosFee {
    /// Amount
    pub amount: Vec<SingleCoin>,
    /// Gas limit
    pub gas_limit: u64,
    /// Payer
    pub payer: Option<String>,
    /// Granter
    pub granter: Option<String>,
}

/// Legacy Amino multisig key.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosLegacyAminoMultisig {
    /// Multisig threshold
    pub threshold: u32,
    /// Public keys which comprise the multisig key
    pub public_keys: Vec<CosmosAny>,
}

/// ModeInfo describes the signing mode of a single or nested multisig signer.
#[derive(Deserialize, Serialize)]
pub enum CosmosModeInfo {
    /// Single signer
    Single { mode: String },
    /// Nested multisig signer
    Multi { modes: Vec<String> },
}

/// SignerInfo describes the public key and signing mode of a single top-level signer.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosSignerInfo {
    /// Signer's public key
    pub public_key: Option<CosmosSignerPublicKey>,
    /// Signing mode
    pub mode_info: CosmosModeInfo,
    /// Account sequence
    pub sequence: u64,
}

/// Signer's public key.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CosmosSignerPublicKey {
    /// Single singer
    Single { key: CosmosAny },
    /// Legacy Amino multisig
    LegacyAminoMultisig { key: CosmosLegacyAminoMultisig },
    /// Other key types
    Any { key: CosmosAny },
}

/// Create a Cosmos parser for `crypto.org`.
pub fn new_crypto_org_parser() -> impl CosmosParser {
    CryptoOrgParser {}
}

// TODO: Add a Cosmos parser for Terra.

/// Cosmos parser trait
pub trait CosmosParser {
    ///
    fn parse_proto_auto_info(&self, hex_string: &str) -> Result<CosmosAuthInfo, CosmosError> {
        todo!();
        // let bytes = hex_decode(hex_string)?;
        // cosmos_sdk_proto::cosmos::tx::v1beta1::AuthInfo::decode(bytes.as_slice())?.try_into()
    }

    fn parse_proto_body(&self);
}

struct NormalParser {}

impl CosmosParser for NormalParser {
    fn parse_proto_body(&self) {}
}

struct CryptoOrgParser {}

impl CosmosParser for CryptoOrgParser {
    fn parse_proto_body(&self) {}
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

    #[test]
    fn test_proto_normal_body_parsing() {
        let body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

        let parser = CosmosParser::new();
        let body = parser.parse_proto_body(body_bytes).unwrap();
    }

    #[test]
    fn test_proto_crypto_org_body_parsing() {
        let body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

        let parser = CosmosParser::new();
        let body = parser.parse_proto_body(body_bytes).unwrap();
    }
}
