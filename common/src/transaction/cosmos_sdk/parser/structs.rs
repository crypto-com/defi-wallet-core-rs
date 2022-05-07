// FIXME:
// It seems that these structs are only used for Cosmos parsing results for now. They could be
// moved to `cosmos_sdk.rs` if reusable.

use crate::transaction::cosmos_sdk::{CosmosError, CosmosSDKMsg, SingleCoin};
use cosmrs::crypto::{LegacyAminoMultisig, PublicKey};
use cosmrs::tx::{mode_info, AuthInfo, Body, Fee, ModeInfo, SignerInfo, SignerPublicKey};
use cosmrs::Any;
use itertools::Itertools;
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
pub struct CosmosTxBody {
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

// gupeng - comment for any message
impl From<Body> for CosmosTxBody {
    fn from(_body: Body) -> Self {
        // gupeng
        todo!()
    }
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

fn format_mode_info(mode_info: ModeInfo) -> String {
    match mode_info {
        ModeInfo::Single(mode_info::Single { mode }) => format!("{mode:?}"),
        ModeInfo::Multi(mode_info::Multi { mode_infos, .. }) => {
            let modes = mode_infos.into_iter().map(format_mode_info).join(", ");
            format!("[{modes}]")
        }
    }
}
