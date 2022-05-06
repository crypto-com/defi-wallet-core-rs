// Cosmos parser is used to deserialize Protobuf or JSON to specified structs. The parsed
// instances could be encoded to a JSON string for display, and `CosmosSDKMsg`s could be used to
// build a new transaction.
// FIXME: It seems that these structs are only used for Cosmos parsing for now. They could be
// moved to `cosmos_sdk.rs` if reusable.

use cosmrs::Any;
use cosmrs::crypto::{LegacyAminoMultisig, PublicKey};
use cosmrs::tx::{mode_info, AuthInfo, Fee, ModeInfo, SignerInfo, SignerPublicKey};
use crate::transaction::cosmos_sdk::{CosmosError, CosmosSDKMsg, SingleCoin};
use crate::utils::hex_decode;
use eyre::WrapErr;
use itertools::Itertools;
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

impl From<Any> for CosmosAny {
    fn from(any: Any) -> Self {
        Self {
            type_url: any.type_url,
            value: base64::encode(any.value),
        }
    }
}

impl TryFrom<PublicKey> for CosmosAny {
    type Error = CosmosError;

    fn try_from(public_key: PublicKey) -> Result<Self, Self::Error> {
        Ok(public_key.to_any()?.into())
    }
}

/// AuthInfo describes the fee and signer modes that are used to sign a transaction.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosAuthInfo {
    /// Fee and gas limit
    pub fee: CosmosFee,
    /// Signing modes for the required signers
    pub signer_infos: Vec<CosmosSignerInfo>,
}

impl TryFrom<AuthInfo> for CosmosAuthInfo {
    type Error = CosmosError;

    fn try_from(auth_info: AuthInfo) -> Result<Self, Self::Error> {
        let signer_infos = auth_info
            .signer_infos
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            fee: auth_info.fee.into(),
            signer_infos,
        })
    }
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

impl From<Fee> for CosmosFee {
    fn from(fee: Fee) -> Self {
        let amount = fee
            .amount
            .into_iter()
            .map(|coin|
            // FIXME:
            // It seems unnecessary to convert to definite Enum value of `SingleCoin`. Since it is
            // only used for display or converting back to `cosmrs::Coin`.
            SingleCoin::Other {
                amount: coin.amount.to_string(),
                denom: coin.denom.to_string(),
            })
            .collect();

        Self {
            amount,
            gas_limit: fee.gas_limit.value(),
            payer: fee.payer.map(|p| p.to_string()),
            granter: fee.granter.map(|g| g.to_string()),
        }
    }
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

impl TryFrom<LegacyAminoMultisig> for CosmosLegacyAminoMultisig {
    type Error = CosmosError;

    fn try_from(multisig: LegacyAminoMultisig) -> Result<Self, CosmosError> {
        Ok(Self {
            threshold: multisig.threshold.into(),
            public_keys: multisig
                .public_keys
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

/// ModeInfo describes the signing mode of a single or nested multisig signer.
#[derive(Deserialize, Serialize)]
pub enum CosmosModeInfo {
    /// Single signer
    Single { mode: String },
    /// Nested multisig signer
    Multi { modes: Vec<String> },
}

impl From<ModeInfo> for CosmosModeInfo {
    fn from(mode_info: ModeInfo) -> Self {
        match mode_info {
            ModeInfo::Single(_) => Self::Single {
                mode: format_mode_info(mode_info),
            },
            ModeInfo::Multi(mode_info::Multi { mode_infos, .. }) => Self::Multi {
                modes: mode_infos.into_iter().map(format_mode_info).collect(),
            },
        }
    }
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

impl TryFrom<SignerInfo> for CosmosSignerInfo {
    type Error = CosmosError;

    fn try_from(signer_info: SignerInfo) -> Result<Self, Self::Error> {
        let public_key = signer_info.public_key.map(TryInto::try_into).transpose()?;

        Ok(Self {
            public_key,
            mode_info: signer_info.mode_info.into(),
            sequence: signer_info.sequence,
        })
    }
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

impl TryFrom<SignerPublicKey> for CosmosSignerPublicKey {
    type Error = CosmosError;

    fn try_from(signer_public_key: SignerPublicKey) -> Result<Self, Self::Error> {
        Ok(match signer_public_key {
            SignerPublicKey::Single(public_key) => Self::Single {
                key: public_key.try_into()?,
            },
            SignerPublicKey::LegacyAminoMultisig(multisig) => Self::LegacyAminoMultisig {
                key: multisig.try_into()?,
            },
            SignerPublicKey::Any(any) => Self::Any { key: any.into() },
        })
    }
}

/// Create a Cosmos parser for `crypto.org` chain.
pub fn new_crypto_org_parser() -> impl CosmosParser {
    CryptoOrgParser {}
}

/// Create a Cosmos parser for `Terra` chain.
pub fn new_terra_parser() -> impl CosmosParser {
    TerraParser {}
}

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

    /// Parse `CosmosBody` from hex data of Protobuf.
    fn parse_proto_body(&self);
    /*
    fn parse_amino_msgs(&self);
    fn parse_amino_fee(&self);
    fn parse_amino_gas_price(&self);
    */
}

struct NormalParser {}

impl CosmosParser for NormalParser {
    fn parse_proto_body(&self) {}
}

struct CryptoOrgParser {}

impl CosmosParser for CryptoOrgParser {
    fn parse_proto_body(&self) {}
}

struct TerraParser {}

impl CosmosParser for TerraParser {
    fn parse_proto_body(&self) {}
}

fn format_mode_info(mode_info: ModeInfo) -> String {
    match mode_info {
        ModeInfo::Single(mode_info::Single { mode }) => format!("{mode:?}"),
        ModeInfo::Multi(mode_info::Multi { mode_infos, .. }) => {
            let modes = mode_infos.into_iter().map(format_mode_info).join(", ");
            format!("[{modes}]")
        }
    }
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
