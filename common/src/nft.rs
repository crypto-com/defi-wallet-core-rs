// ! NFT module support

use crate::{msg_wrapper, proto, AccountId, ErrorReport, Msg, Result};
use eyre::WrapErr;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DenomId(String);
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DenomName(String);
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct TokenId(String);
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct TokenUri(String);

const MIN_DENOM_LEN: usize = 3;
const MAX_DENOM_LEN: usize = 64;
const MAX_TOKEN_URI_LEN: usize = 256;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    /// Invalid DenomId
    #[error("invalid DenomId: {id:?}")]
    DenomId { id: String },
    /// Invalid DenomName
    #[error("invalid DenomName: {name:?}")]
    DenomName { name: String },
    /// Invalid TokenId
    #[error("invalid TokenId: {id:?}")]
    TokenId { id: String },
    /// Invalid TokenUri
    #[error("invalid TokenUri: {uri:?}")]
    TokenUri { uri: String },
}

impl Display for DenomId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for DenomName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for TokenId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for TokenUri {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DenomId {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<DenomId> {
        match s.len() {
            MIN_DENOM_LEN..=MAX_DENOM_LEN => {
                for c in s.chars() {
                    if !c.is_ascii_alphanumeric() || !c.is_lowercase() {
                        return Err(Error::DenomId { id: s.to_owned() }).wrap_err_with(|| {
                            format!(
                                "the denom({}) only accepts lowercase alphanumeric characters",
                                s.to_owned()
                            )
                        });
                    }
                }

                if s.chars().next().unwrap().is_ascii_alphabetic() {
                    Ok(Self(s.to_owned()))
                } else {
                    Err(Error::DenomId { id: s.to_owned() }).wrap_err_with(|| {
                        format!(
                            "the denom({}) only begins with an English letter",
                            s.to_owned()
                        )
                    })
                }
            }
            _ => Err(Error::DenomId { id: s.to_owned() }).wrap_err_with(|| {
                format!(
                    "the length of denom({}) only accepts value [{}, {}]",
                    s.to_owned(),
                    MIN_DENOM_LEN,
                    MAX_DENOM_LEN
                )
            }),
        }
    }
}

impl FromStr for DenomName {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<DenomName> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        if s.len() == 0 {
            Err(Error::DenomName { name: s.to_owned() })
                .wrap_err_with(|| format!("denom name({}) can not be space", s.to_owned()))
        } else {
            Ok(Self(s.to_owned()))
        }
    }
}

impl FromStr for TokenId {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<TokenId> {
        match s.len() {
            MIN_DENOM_LEN..=MAX_DENOM_LEN => {
                for c in s.chars() {
                    if !c.is_ascii_alphanumeric() || !c.is_lowercase() {
                        return Err(Error::TokenId { id: s.to_owned() }).wrap_err_with(|| {
                            format!(
                                "nft id({}) only accepts lowercase alphanumeric characters",
                                s.to_owned()
                            )
                        });
                    }
                }

                if s.chars().next().unwrap().is_ascii_alphabetic() {
                    Ok(Self(s.to_owned()))
                } else {
                    Err(Error::TokenId { id: s.to_owned() }).wrap_err_with(|| {
                        format!(
                            "nft id({}) only begins with an English letter",
                            s.to_owned()
                        )
                    })
                }
            }
            _ => Err(Error::TokenId { id: s.to_owned() }).wrap_err_with(|| {
                format!(
                    "the length of nft id({}) only accepts value [{}, {}]",
                    s.to_owned(),
                    MIN_DENOM_LEN,
                    MAX_DENOM_LEN
                )
            }),
        }
    }
}

impl FromStr for TokenUri {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<TokenUri> {
        match s.len() {
            0..=MAX_TOKEN_URI_LEN => Ok(Self(s.to_owned())),
            _ => Err(Error::TokenUri { uri: s.to_owned() }).wrap_err_with(|| {
                format!(
                    "the length of nft uri({}) only accepts value [0, {}]",
                    s.to_owned(),
                    MAX_TOKEN_URI_LEN
                )
            }),
        }
    }
}

msg_wrapper! {
    // MsgIssueDenom defines an SDK message for creating a new denom.
    proto::chainmain::nft::v1::MsgIssueDenom => pub struct MsgIssueDenom {
        pub id: DenomId,
        pub name: DenomName,
        pub schema: String,
        pub sender: AccountId,
    }
}

msg_wrapper! {
    // MsgMintNft defines an SDK message for creating a new NFT.
    proto::chainmain::nft::v1::MsgMintNft =>  pub struct MsgMintNft {
        pub id: TokenId,
        pub denom_id: DenomId,
        pub name: DenomName,
        pub uri: TokenUri,
        pub data: String,
        pub sender: AccountId,
        pub recipient: AccountId,
    }
}

msg_wrapper! {
   // MsgEditNft defines an SDK message for editing a nft.
   proto::chainmain::nft::v1::MsgEditNft => pub struct MsgEditNft {
       pub id: TokenId,
       pub denom_id: DenomId,
       pub name: DenomName,
       pub uri: TokenUri,
       pub data: String,
       pub sender: AccountId,
   }
}

msg_wrapper! {
   // MsgTransferNft defines an SDK message for transferring an NFT to recipient.
   proto::chainmain::nft::v1::MsgTransferNft => pub struct MsgTransferNft {
       pub id: TokenId,
       pub denom_id: DenomId,
       pub sender: AccountId,
       pub recipient: AccountId,
   }
}

msg_wrapper! {
   // MsgBurnNft defines an SDK message for burning a NFT.
   proto::chainmain::nft::v1::MsgBurnNft => pub struct MsgBurnNft {
       pub id: TokenId,
       pub denom_id: DenomId,
       pub sender: AccountId,
   }
}
