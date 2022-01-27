//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use crate::crypto::Key;
use crate::protocol::{
    Metadata, PeerMetadata, SessionParams, SessionRequest, SessionUpdate, Topic,
};
use crate::uri::Uri;
use ethers::prelude::Address;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use url::form_urlencoded::Serializer;
use url::Url;

/// The WalletConnect 1.0 session information
/// based on the initial request-response: https://docs.walletconnect.com/tech-spec#session-request
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// if the wallet approved the connection
    pub connected: bool,
    /// the accounts returned by the wallet
    pub accounts: Vec<Address>,
    /// the chain id returned by the wallet
    pub chain_id: Option<u64>,
    /// the bridge server URL
    pub bridge: Url,
    /// the secret key used in encrypting wallet requests
    /// and decrypting wallet responses as per WalletConnect 1.0
    pub key: Key,
    /// this is the client's randomly generated ID
    pub client_id: Topic,
    /// the client metadata (that will be presented to the wallet in the initial request)
    pub client_meta: Metadata,
    /// the wallet's ID
    pub peer_id: Option<Topic>,
    /// the wallet's metadata
    pub peer_meta: Option<PeerMetadata>,
    /// the one-time request ID
    pub handshake_topic: Topic,
}

impl Session {
    /// generate the session URI: https://docs.walletconnect.com/tech-spec#requesting-connection
    /// https://eips.ethereum.org/EIPS/eip-1328
    pub fn uri(&self) -> Uri {
        Uri::parse(&format!(
            "wc:{}@1?{}",
            self.handshake_topic,
            Serializer::new(String::new())
                .append_pair("bridge", self.bridge.as_str())
                .append_pair("key", self.key.display().expose_secret())
                .finish()
        ))
        .expect("WalletConnect URIs from sessions are always valid")
    }

    /// generates a session request from the session: https://docs.walletconnect.com/tech-spec#session-request
    pub fn request(&self) -> SessionRequest {
        SessionRequest {
            peer_id: self.client_id.clone(),
            peer_meta: self.client_meta.clone(),
            chain_id: self.chain_id,
        }
    }

    /// updates the session details from the response
    pub fn apply(&mut self, params: SessionParams) {
        self.connected = params.approved;
        self.accounts = params.accounts;
        self.chain_id = Some(params.chain_id);
        self.peer_id = Some(params.peer_id);
        self.peer_meta = Some(params.peer_meta);
    }

    /// updates the session details from the session update: https://docs.walletconnect.com/tech-spec#session-update
    pub fn update(&mut self, update: SessionUpdate) {
        self.connected = update.approved;
        self.accounts = update.accounts;
        self.chain_id = Some(update.chain_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new_topic_is_random() {
        assert_ne!(Topic::new(), Topic::new());
    }

    #[test]
    fn zero_topic() {
        assert_eq!(
            json!(Topic::zero()),
            json!("00000000-0000-0000-0000-000000000000")
        );
    }

    #[test]
    fn topic_serialization() {
        let topic = Topic::new();
        let serialized = serde_json::to_string(&topic).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(topic, deserialized);
    }
}
