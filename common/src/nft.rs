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

impl AsRef<str> for DenomId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<str> for DenomName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<str> for TokenId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<str> for TokenUri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for DenomId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl Display for DenomName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl Display for TokenId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl Display for TokenUri {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for DenomId {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<DenomId> {
        match s.len() {
            MIN_DENOM_LEN..=MAX_DENOM_LEN => {
                if s.chars()
                    .any(|c| !c.is_ascii_alphanumeric() || c.is_ascii_uppercase())
                {
                    return Err(Error::DenomId { id: s.to_owned() }).wrap_err_with(|| {
                        format!(
                            "the denom({}) only accepts lowercase alphanumeric characters",
                            s.to_owned()
                        )
                    });
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

        if s.is_empty() {
            Err(Error::DenomName { name: s.to_owned() })
                .wrap_err_with(|| format!("denom name({}) can not be space", s.to_owned()))
        } else {
            Ok(Self(s))
        }
    }
}

impl FromStr for TokenId {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<TokenId> {
        match s.len() {
            MIN_DENOM_LEN..=MAX_DENOM_LEN => {
                if s.chars()
                    .any(|c| !c.is_ascii_alphanumeric() || c.is_ascii_uppercase())
                {
                    return Err(Error::TokenId { id: s.to_owned() }).wrap_err_with(|| {
                        format!(
                            "nft id({}) only accepts lowercase alphanumeric characters",
                            s.to_owned()
                        )
                    });
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

#[cfg(test)]
mod test {
    use super::*;
    use cosmrs::{
        crypto::secp256k1,
        tx::{self, Fee, Msg, SignDoc, SignerInfo, Tx},
        Coin,
    };

    #[test]
    fn test_denom_id() {
        let id = " ";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of denom({}) only accepts value [{}, {}]",
                id, MIN_DENOM_LEN, MAX_DENOM_LEN
            )
        );

        let id = (0..MAX_DENOM_LEN + 1).map(|_| "a").collect::<String>();
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of denom({}) only accepts value [{}, {}]",
                id, MIN_DENOM_LEN, MAX_DENOM_LEN
            )
        );

        let id = "a";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of denom({}) only accepts value [{}, {}]",
                id, MIN_DENOM_LEN, MAX_DENOM_LEN
            )
        );

        let id = "1bc";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!("the denom({}) only begins with an English letter", id)
        );

        let id = "Abc";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the denom({}) only accepts lowercase alphanumeric characters",
                id
            )
        );

        let id = "ab1";
        let result = id.parse::<DenomId>();
        assert_eq!(result.unwrap().as_ref(), "ab1");

        let id = "testdenomid";
        let result = id.parse::<DenomId>();
        assert_eq!(result.unwrap().as_ref(), "testdenomid");
    }

    #[test]
    fn test_denom_name() {
        let name = " ";
        let error = name.parse::<DenomName>().unwrap_err().to_string();
        assert_eq!(error, "denom name() can not be space".to_string());

        let name = " a   b    c   ";
        let result = name.parse::<DenomName>();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().as_ref(), "abc");

        let name = "testdenomname";
        let result = name.parse::<DenomName>();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().as_ref(), "testdenomname");
    }

    #[test]
    fn test_token_id() {
        let id = " ";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of nft id({}) only accepts value [{}, {}]",
                id, MIN_DENOM_LEN, MAX_DENOM_LEN
            )
        );

        let id = (0..MAX_DENOM_LEN + 1).map(|_| "a").collect::<String>();
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of nft id({}) only accepts value [{}, {}]",
                id, MIN_DENOM_LEN, MAX_DENOM_LEN
            )
        );

        let id = "a";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of nft id({}) only accepts value [{}, {}]",
                id, MIN_DENOM_LEN, MAX_DENOM_LEN
            )
        );

        let id = "1bc";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!("nft id({}) only begins with an English letter", id)
        );

        let id = "Abc";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "nft id({}) only accepts lowercase alphanumeric characters",
                id
            )
        );

        let id = "ab1";
        let result = id.parse::<TokenId>();
        assert_eq!(result.unwrap().as_ref(), "ab1");

        let id = "testtokenid";
        let result = id.parse::<TokenId>();
        assert_eq!(result.unwrap().as_ref(), "testtokenid");
    }

    #[test]
    fn test_token_uri() {
        let uri = (0..MAX_TOKEN_URI_LEN + 1).map(|_| "a").collect::<String>();
        let error = uri.parse::<TokenUri>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of nft uri({}) only accepts value [0, {}]",
                uri, MAX_TOKEN_URI_LEN
            )
        );

        let uri = "";
        let result = uri.parse::<TokenUri>();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().as_ref(), "");

        let uri = "testuri";
        let result = uri.parse::<TokenUri>();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().as_ref(), "testuri");
    }

    #[test]
    fn test_nft_msg() {
        // Generate sender private key.
        // In real world usage, this account would need to be funded before use.
        let sender_private_key = secp256k1::SigningKey::random();
        let sender_public_key = sender_private_key.public_key();
        let sender_account_id = sender_public_key.account_id("chainmain").unwrap();
        let recipient_account_id = "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy" // signer1
            .parse::<AccountId>()
            .unwrap();

        let msg_issue_denom = MsgIssueDenom {
            id: "testdenomid".parse::<DenomId>().unwrap(),
            name: "testdenomname".parse::<DenomName>().unwrap(),
            schema: r#"
                    {
                        "title":"Asset Metadata",
                        "type":"object",
                        "properties":{
                            "name":{
                                "type":"string",
                                "description":"testidentity"
                            },
                            "description":{
                                "type":"string",
                                "description":"testdescription"
                            },
                            "image":{
                                "type":"string",
                                "description":"testdescription"
                            }
                        }
                    }"#
            .to_string(),
            sender: sender_account_id.clone(),
        };

        let msg_mint_nft = MsgMintNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            name: "testtokenid".parse::<DenomName>().unwrap(),
            uri: "testuri".parse::<TokenUri>().unwrap(),
            data: "".to_owned(),
            sender: sender_account_id.clone(),
            recipient: recipient_account_id.clone(),
        };

        let msg_edit_nft = MsgEditNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            name: "newname".parse::<DenomName>().unwrap(),
            uri: "newuri".parse::<TokenUri>().unwrap(),
            data: "".to_owned(),
            sender: sender_account_id.clone(),
        };

        let msg_transfer_nft = MsgTransferNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            sender: sender_account_id.clone(),
            recipient: recipient_account_id.clone(),
        };

        let msg_burn_nft = MsgBurnNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            sender: sender_account_id.clone(),
        };

        create_nft_msg(&sender_private_key, msg_issue_denom).unwrap();
        create_nft_msg(&sender_private_key, msg_mint_nft).unwrap();
        create_nft_msg(&sender_private_key, msg_edit_nft).unwrap();
        create_nft_msg(&sender_private_key, msg_transfer_nft).unwrap();
        create_nft_msg(&sender_private_key, msg_burn_nft).unwrap();
    }

    fn create_nft_msg(private_key: &secp256k1::SigningKey, msg: impl Msg) -> Result<()> {
        ///////////////////////////
        // Building transactions //
        ///////////////////////////
        // We'll be doing a simple send transaction.
        // First we'll create a "Coin" amount to be sent, in this case 1 million cro.
        let amount = Coin {
            amount: 1_000_000u64.into(),
            denom: "basecro".parse()?,
        };

        // Transaction metadata: chain, account, sequence, gas, fee, timeout, and memo.
        let chain_id = "chaintest".parse()?;
        let account_number = 1;
        let sequence_number = 0;
        let gas = 100_000;
        let timeout_height = 9001u16;
        let memo = "example memo";

        // Create transaction body from the MsgIssueDenom, memo, and timeout height.
        let tx_body = tx::Body::new(vec![msg.to_any()?], memo, timeout_height);

        // Create signer info from public key and sequence number.
        // This uses a standard "direct" signature from a single signer.
        let signer_info =
            SignerInfo::single_direct(Some(private_key.public_key()), sequence_number);

        // Compute auth info from signer info by associating a fee.
        let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount, gas));

        //////////////////////////
        // Signing transactions //
        //////////////////////////

        // The "sign doc" contains a message to be signed.
        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

        // Sign the "sign doc" with the sender's private key, producing a signed raw transaction.
        let tx_signed = sign_doc.sign(&private_key)?;

        // Serialize the raw transaction as bytes (i.e. `Vec<u8>`).
        let tx_bytes = tx_signed.to_bytes()?;

        //////////////////////////
        // Parsing transactions //
        //////////////////////////

        // Parse the serialized bytes from above into a `cosmrs::Tx`
        let tx_parsed = Tx::from_bytes(&tx_bytes)?;
        assert_eq!(tx_parsed.body, tx_body);
        assert_eq!(tx_parsed.auth_info, auth_info);

        Ok(())
    }
}
