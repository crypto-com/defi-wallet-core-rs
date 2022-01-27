//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use crate::crypto::Key;
use crate::protocol::Topic;
use qrcodegen::{QrCode, QrCodeEcc};
use thiserror::Error;
use url::Url;

/// The WalletConnect 1.0 connection information that's to be passed to the wallets
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri {
    handshake_topic: Topic,
    version: u64,
    bridge: Url,
    key: Key,
    url: Url,
}

const VERSION: u64 = 1;

/// Prints the given QrCode object to the console.
fn print_qr(qr: &QrCode) {
    let border: i32 = 4;
    for y in -border..qr.size() + border {
        for x in -border..qr.size() + border {
            let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
    println!();
}

impl Uri {
    /// prints the URI + its QR code representation in a console
    pub fn print_qr_uri(&self) {
        println!("session uri: {}", self.url);
        if let Ok(qr) = QrCode::encode_text(self.url.as_str(), QrCodeEcc::Low) {
            print_qr(&qr);
        } else {
            eprintln!("failed to encode URI as a QR code");
        }
    }

    /// parse the given URI from a string
    pub fn parse(uri: impl AsRef<str>) -> Result<Self, InvalidSessionUri> {
        let url = Url::parse(uri.as_ref())?;
        if url.scheme() != "wc" {
            return Err(InvalidSessionUri);
        }

        let mut path = url.path().splitn(2, '@');
        let handshake_topic = path.next().ok_or(InvalidSessionUri)?.parse()?;
        let version = path.next().ok_or(InvalidSessionUri)?.parse()?;
        if version != VERSION {
            return Err(InvalidSessionUri);
        }

        let mut bridge: Option<Url> = None;
        let mut key: Option<Key> = None;
        for (name, value) in url.query_pairs() {
            match &*name {
                "bridge" => bridge = Some(value.parse()?),
                "key" => key = Some(value.parse()?),
                _ => return Err(InvalidSessionUri),
            }
        }

        Ok(Uri {
            handshake_topic,
            version,
            bridge: bridge.ok_or(InvalidSessionUri)?,
            key: key.ok_or(InvalidSessionUri)?,
            url,
        })
    }

    /// the topic used in the initial session request
    pub fn handshake_topic(&self) -> &Topic {
        &self.handshake_topic
    }

    /// version -- 1 for 1.0
    pub fn version(&self) -> u64 {
        self.version
    }

    /// the bridge server URL
    pub fn bridge(&self) -> &Url {
        &self.bridge
    }

    /// the symmetric key used to encrypt the requests/responses
    /// between the client and wallet
    pub fn key(&self) -> &Key {
        &self.key
    }

    /// returns the handshake topic, the bridge server URL, and the key
    pub fn into_parts(self) -> (Topic, Url, Key) {
        (self.handshake_topic, self.bridge, self.key)
    }

    /// returns the full URI as a URL string
    pub fn as_url(&self) -> &Url {
        &self.url
    }
}

/// The error type for invalid session URIs
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("session URI is invalid")]
pub struct InvalidSessionUri;

macro_rules! impl_invalid_session_uri_from {
    ($err:ty) => {
        impl From<$err> for InvalidSessionUri {
            fn from(_: $err) -> Self {
                InvalidSessionUri
            }
        }
    };
}

impl_invalid_session_uri_from!(ethers::utils::hex::FromHexError);
impl_invalid_session_uri_from!(std::num::ParseIntError);
impl_invalid_session_uri_from!(url::ParseError);
impl_invalid_session_uri_from!(uuid::Error);
