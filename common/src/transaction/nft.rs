// ! NFT module support

use crate::{
    build_signed_single_msg_tx, msg_wrapper, proto, AccountId, CosmosError, CosmosSDKMsg,
    CosmosSDKTxInfo, ErrorReport, Msg, Result, SecretKey,
};
use eyre::WrapErr;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

/// The denomination ID of the NFT
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct DenomId(String);
/// The denomination name of the NFT
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct DenomName(String);
/// The unique ID of the NFT
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TokenId(String);
/// The URI pointing to a JSON object that contains subsequent tokenData information off-chain
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TokenUri(String);

const MIN_DENOM_LEN: usize = 3;
const MAX_DENOM_LEN: usize = 64;
const MAX_TOKEN_URI_LEN: usize = 256;

trait Helper {
    fn type_name() -> &'static str;
    fn new(s: &str) -> Self;
    fn err(e: &str) -> Error;
}

trait Validate {
    fn validate<T: Helper + Display>(s: &str) -> Result<T>;
}

/// NFT metadata parse errors
#[derive(Clone, Debug, Error, PartialEq, Eq)]
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

impl Helper for DenomName {
    fn type_name() -> &'static str {
        "DenomName"
    }
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn err(s: &str) -> Error {
        Error::DenomName { name: s.to_owned() }
    }
}

impl Helper for TokenId {
    fn type_name() -> &'static str {
        "TokenId"
    }
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn err(s: &str) -> Error {
        Error::TokenId { id: s.to_owned() }
    }
}

impl Helper for DenomId {
    fn type_name() -> &'static str {
        "DenomId"
    }
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn err(s: &str) -> Error {
        Error::DenomId { id: s.to_owned() }
    }
}

impl Helper for TokenUri {
    fn type_name() -> &'static str {
        "TokenUri"
    }
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
    fn err(s: &str) -> Error {
        Error::TokenUri { uri: s.to_owned() }
    }
}

impl Validate for DenomName {
    fn validate<T: Helper + Display>(s: &str) -> Result<T> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        if s.is_empty() {
            Err(T::err(&s)).wrap_err_with(|| format!("{}({}) can not be space", T::type_name(), s))
        } else {
            Ok(T::new(&s))
        }
    }
}

impl Validate for TokenId {
    fn validate<T: Helper + Display>(s: &str) -> Result<T> {
        validate_id::<T>(s)
    }
}

impl Validate for DenomId {
    fn validate<T: Helper + Display>(s: &str) -> Result<T> {
        validate_id::<T>(s)
    }
}

impl Validate for TokenUri {
    fn validate<T: Helper + Display>(s: &str) -> Result<T> {
        match s.len() {
            0..=MAX_TOKEN_URI_LEN => Ok(T::new(s)),
            _ => Err(T::err(s)).wrap_err_with(|| {
                format!(
                    "the length of {}({}) only accepts value [0, {}]",
                    T::type_name(),
                    s,
                    MAX_TOKEN_URI_LEN
                )
            }),
        }
    }
}

fn validate_id<T: Helper + Display>(s: &str) -> Result<T> {
    match s.len() {
        MIN_DENOM_LEN..=MAX_DENOM_LEN => {
            if s.chars()
                .any(|c| !c.is_ascii_alphanumeric() || c.is_ascii_uppercase())
            {
                return Err(T::err(s)).wrap_err_with(|| {
                    format!(
                        "the {}({}) only accepts lowercase alphanumeric characters",
                        T::type_name(),
                        T::new(s)
                    )
                });
            }

            if s.chars().next().unwrap().is_ascii_alphabetic() {
                Ok(T::new(s))
            } else {
                Err(T::err(s)).wrap_err_with(|| {
                    format!(
                        "the {}({}) only begins with an English letter",
                        T::type_name(),
                        T::new(s)
                    )
                })
            }
        }
        _ => Err(T::err(s)).wrap_err_with(|| {
            format!(
                "the length of {}({}) only accepts value [{}, {}]",
                T::type_name(),
                T::new(s),
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        }),
    }
}

impl FromStr for DenomId {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<DenomId> {
        DenomId::validate(s)
    }
}

impl FromStr for DenomName {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<DenomName> {
        DenomName::validate(s)
    }
}

impl FromStr for TokenId {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<TokenId> {
        TokenId::validate(s)
    }
}

impl FromStr for TokenUri {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<TokenUri> {
        TokenUri::validate(s)
    }
}

msg_wrapper! {
    /// MsgIssueDenom defines an SDK message for creating a new denom.
    proto::chainmain::nft::v1::MsgIssueDenom => pub struct MsgIssueDenom {
        /// The denomination ID of the NFT, necessary as multiple denominations are able to be represented on each chain
        pub id: DenomId,
        /// The denomination name of the NFT, necessary as multiple denominations are able to be represented on each chain.
        pub name: DenomName,
        /// The account address of the user creating the denomination.
        pub schema: String,
        /// NFT specifications defined under this category
        pub sender: AccountId,
    }
}

msg_wrapper! {
    /// MsgMintNft defines an SDK message for creating a new NFT.
    proto::chainmain::nft::v1::MsgMintNft =>  pub struct MsgMintNft {
        /// The unique ID of the NFT being minted
        pub id: TokenId,
        /// The unique ID of the denomination.
        pub denom_id: DenomId,
        /// The name of the NFT being minted.
        pub name: String,
        /// The URI pointing to a JSON object that contains subsequent tokenData information off-chain
        pub uri: TokenUri,
        /// The data of the NFT.
        pub data: String,
        /// The sender of the Message
        pub sender: AccountId,
        /// The recipient of the new NFT
        pub recipient: AccountId,
    }
}

msg_wrapper! {
   /// MsgEditNft defines an SDK message for editing a nft.
   proto::chainmain::nft::v1::MsgEditNft => pub struct MsgEditNft {
       /// The unique ID of the NFT being edited.
       pub id: TokenId,
       /// The unique ID of the denomination, necessary as multiple denominations are able to be represented on each chain.
       pub denom_id: DenomId,
       /// The name of the NFT being edited.
       pub name: String,
       /// The URI pointing to a JSON object that contains subsequent tokenData information off-chain
       pub uri: TokenUri,
       /// The data of the NFT
       pub data: String,
       /// The creator of the message
       pub sender: AccountId,
   }
}

msg_wrapper! {
   /// MsgTransferNft defines an SDK message for transferring an NFT to recipient.
   proto::chainmain::nft::v1::MsgTransferNft => pub struct MsgTransferNft {
       /// The unique ID of the NFT being transferred.
       pub id: TokenId,
       /// The unique ID of the denomination, necessary as multiple denominations are able to be represented on each chain.
       pub denom_id: DenomId,
       /// The account address of the user sending the NFT.
       pub sender: AccountId,
       /// The account address who will receive the NFT as a result of the transfer transaction.
       pub recipient: AccountId,
   }
}

msg_wrapper! {
   /// MsgBurnNft defines an SDK message for burning a NFT.
   proto::chainmain::nft::v1::MsgBurnNft => pub struct MsgBurnNft {
       /// The ID of the Token.
       pub id: TokenId,
       /// The Denom ID of the Token.
       pub denom_id: DenomId,
       /// The account address of the user burning the token.
       pub sender: AccountId,
   }
}

/// creates the signed transaction
/// for `MsgIssueDenom` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
pub fn get_nft_issue_denom_signed_tx(
    tx_info: CosmosSDKTxInfo,
    secret_key: Arc<SecretKey>,
    id: String,
    name: String,
    schema: String,
) -> Result<Vec<u8>, CosmosError> {
    build_signed_single_msg_tx(
        tx_info,
        CosmosSDKMsg::NftIssueDenom { id, name, schema },
        secret_key,
    )
}

/// creates the signed transaction
/// for `MsgMintNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
#[allow(clippy::too_many_arguments)]
pub fn get_nft_mint_signed_tx(
    tx_info: CosmosSDKTxInfo,
    secret_key: Arc<SecretKey>,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    recipient: String,
) -> Result<Vec<u8>, CosmosError> {
    build_signed_single_msg_tx(
        tx_info,
        CosmosSDKMsg::NftMint {
            id,
            denom_id,
            name,
            uri,
            data,
            recipient,
        },
        secret_key,
    )
}

/// creates the signed transaction
/// for `MsgEditNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
pub fn get_nft_edit_signed_tx(
    tx_info: CosmosSDKTxInfo,
    secret_key: Arc<SecretKey>,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
) -> Result<Vec<u8>, CosmosError> {
    build_signed_single_msg_tx(
        tx_info,
        CosmosSDKMsg::NftEdit {
            id,
            denom_id,
            name,
            uri,
            data,
        },
        secret_key,
    )
}

/// creates the signed transaction
/// for `MsgTransferNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
pub fn get_nft_transfer_signed_tx(
    tx_info: CosmosSDKTxInfo,
    secret_key: Arc<SecretKey>,
    id: String,
    denom_id: String,
    recipient: String,
) -> Result<Vec<u8>, CosmosError> {
    build_signed_single_msg_tx(
        tx_info,
        CosmosSDKMsg::NftTransfer {
            id,
            denom_id,
            recipient,
        },
        secret_key,
    )
}

/// creates the signed transaction
/// for `MsgBurnNft` from the Chainmain nft module
/// wasm-bindgen only supports the C-style enums,
/// hences this duplicate function
pub fn get_nft_burn_signed_tx(
    tx_info: CosmosSDKTxInfo,
    secret_key: Arc<SecretKey>,
    id: String,
    denom_id: String,
) -> Result<Vec<u8>, CosmosError> {
    build_signed_single_msg_tx(tx_info, CosmosSDKMsg::NftBurn { id, denom_id }, secret_key)
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
                "the length of {}({}) only accepts value [{}, {}]",
                DenomId::type_name(),
                id,
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        );

        let id = (0..MAX_DENOM_LEN + 1).map(|_| "a").collect::<String>();
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of {}({}) only accepts value [{}, {}]",
                DenomId::type_name(),
                id,
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        );

        let id = "a";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of {}({}) only accepts value [{}, {}]",
                DenomId::type_name(),
                id,
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        );

        let id = "1bc";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the {}({}) only begins with an English letter",
                DenomId::type_name(),
                id
            )
        );

        let id = "Abc";
        let error = id.parse::<DenomId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the {}({}) only accepts lowercase alphanumeric characters",
                DenomId::type_name(),
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
        assert_eq!(
            error,
            format!("{}() can not be space", DenomName::type_name())
        );

        let name = " a   b    c   ";
        let result = name.parse::<DenomName>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "abc");

        let name = "testdenomname";
        let result = name.parse::<DenomName>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "testdenomname");
    }

    #[test]
    fn test_token_id() {
        let id = " ";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of {}({}) only accepts value [{}, {}]",
                TokenId::type_name(),
                id,
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        );

        let id = (0..MAX_DENOM_LEN + 1).map(|_| "a").collect::<String>();
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of {}({}) only accepts value [{}, {}]",
                TokenId::type_name(),
                id,
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        );

        let id = "a";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the length of {}({}) only accepts value [{}, {}]",
                TokenId::type_name(),
                id,
                MIN_DENOM_LEN,
                MAX_DENOM_LEN
            )
        );

        let id = "1bc";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the {}({}) only begins with an English letter",
                TokenId::type_name(),
                id
            )
        );

        let id = "Abc";
        let error = id.parse::<TokenId>().unwrap_err().to_string();
        assert_eq!(
            error,
            format!(
                "the {}({}) only accepts lowercase alphanumeric characters",
                TokenId::type_name(),
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
                "the length of {}({}) only accepts value [0, {}]",
                TokenUri::type_name(),
                uri,
                MAX_TOKEN_URI_LEN
            )
        );

        let uri = "";
        let result = uri.parse::<TokenUri>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "");

        let uri = "testuri";
        let result = uri.parse::<TokenUri>();
        assert!(result.is_ok());
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
            name: "".to_owned(),
            uri: "testuri".parse::<TokenUri>().unwrap(),
            data: "".to_owned(),
            sender: sender_account_id.clone(),
            recipient: recipient_account_id.clone(),
        };

        let msg_edit_nft = MsgEditNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            name: "newname".to_owned(),
            uri: "newuri".parse::<TokenUri>().unwrap(),
            data: "".to_owned(),
            sender: sender_account_id.clone(),
        };

        let msg_transfer_nft = MsgTransferNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            sender: sender_account_id.clone(),
            recipient: recipient_account_id,
        };

        let msg_burn_nft = MsgBurnNft {
            id: "testtokenid".parse::<TokenId>().unwrap(),
            denom_id: "testdenomid".parse::<DenomId>().unwrap(),
            sender: sender_account_id,
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
        let gas = 100_000u64;
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
        let tx_signed = sign_doc.sign(private_key)?;

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
