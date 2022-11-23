// FIXME:
// It seems that these structs are only used for Cosmos parsing results for now. They could be
// moved to `cosmos_sdk.rs` if reusable.

use crate::transaction::cosmos_sdk::{CosmosError, SingleCoin};
use cosmrs::crypto::{LegacyAminoMultisig, PublicKey};
use cosmrs::tx::{mode_info, AuthInfo, Body, Fee, ModeInfo, SignerInfo, SignerPublicKey};
use itertools::Itertools;
use serde::{de, Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

mod cosmos_raw_msg;
pub use cosmos_raw_msg::*;

/// Any contains arbitrary data along with a URL that describes the data type.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CosmosAny {
    /// URL of data type
    pub type_url: String,
    /// Base64 encoded data
    pub value: String,
}

impl From<cosmrs::Any> for CosmosAny {
    fn from(any: cosmrs::Any) -> Self {
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
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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

/// Fee includes the amount of coins paid in fees and the maximum gas to be used by the transaction.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CosmosFee {
    /// Amount
    pub amount: Vec<SingleCoin>,
    /// Gas limit
    #[serde(deserialize_with = "deserialize_from_str")]
    pub gas_limit: u64,
    /// Payer
    pub payer: Option<String>,
    /// Granter
    pub granter: Option<String>,
}

impl From<Fee> for CosmosFee {
    fn from(fee: Fee) -> Self {
        let amount = fee.amount.into_iter().map(Into::into).collect();

        Self {
            amount,
            gas_limit: fee.gas_limit,
            payer: fee.payer.map(|p| p.to_string()),
            granter: fee.granter.map(|g| g.to_string()),
        }
    }
}

/// Block height.
#[derive(Clone, Deserialize, Serialize)]
pub struct CosmosHeight {
    /// Epoch
    pub revision_number: u64,
    /// Height
    pub revision_height: u64,
}

/// Legacy Amino multisig key.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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
            threshold: multisig.threshold,
            public_keys: multisig
                .public_keys
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

/// ModeInfo describes the signing mode of a single or nested multisig signer.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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

/// Body of a transaction that all signers sign over.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CosmosTxBody {
    /// Message list
    pub messages: Vec<CosmosRawMsg>,
    /// Memo
    pub memo: String,
    /// Timeout
    pub timeout_height: u64,
    /// Extension options
    pub extension_options: Vec<CosmosAny>,
    /// Non critical extension options
    pub non_critical_extension_options: Vec<CosmosAny>,
}

// This conversion directly transforms messages to type `CosmosRawMsg::Any`. The detailed messages
// (as `BankSend`) should be transformed in specified parser.
impl From<Body> for CosmosTxBody {
    fn from(body: Body) -> Self {
        let messages = body
            .messages
            .into_iter()
            .map(|any| CosmosRawMsg::Any {
                type_url: any.type_url,
                value: any.value,
            })
            .collect();
        let extension_options = body.extension_options.into_iter().map(Into::into).collect();
        let non_critical_extension_options = body
            .non_critical_extension_options
            .into_iter()
            .map(Into::into)
            .collect();

        Self {
            messages,
            memo: body.memo,
            timeout_height: body.timeout_height.value(),
            extension_options,
            non_critical_extension_options,
        }
    }
}

fn deserialize_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
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
