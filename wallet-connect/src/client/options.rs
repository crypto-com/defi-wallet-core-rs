//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use super::session::Session;
use crate::crypto::Key;
use crate::protocol::{Metadata, Topic};
use crate::uri::Uri;
use url::Url;

/// The provided WalletConnect connection information
#[derive(Clone, Debug)]
pub enum Connection {
    /// only the bridge server URL is provided
    /// (the full URI will need to be generated)
    Bridge(Url),
    /// the full URI is provided
    Uri(Uri),
}

impl Default for Connection {
    fn default() -> Self {
        Connection::Bridge(Url::parse("https://l.bridge.walletconnect.org").unwrap())
    }
}

/// The WalletConnect 1.0 configuration
#[derive(Clone, Debug)]
pub struct Options {
    /// the client metadata (will be presented to the wallet)
    pub meta: Metadata,
    /// the provided connection information
    pub connection: Connection,
    /// the chain id (otherwise the chain id is retrieved from the wallet)
    /// TODO: right now, it seems this is not checked against the wallet's chain id
    pub chain_id: Option<u64>,
}

impl Options {
    /// creates a new configuration with a default bridge server URL
    pub fn new(meta: Metadata) -> Self {
        Options {
            meta,
            connection: Connection::default(),
            chain_id: None,
        }
    }

    /// creates a new configuration with a provided URI
    pub fn with_uri(meta: Metadata, uri: Uri) -> Self {
        Options {
            meta,
            connection: Connection::Uri(uri),
            chain_id: None,
        }
    }

    /// creates a new session from the configuration
    pub fn create_session(self) -> Session {
        let client_meta = self.meta;
        let (handshake_topic, bridge, key) = match self.connection {
            Connection::Bridge(bridge) => (Topic::new(), bridge, Key::random()),
            Connection::Uri(uri) => uri.into_parts(),
        };
        let chain_id = self.chain_id;

        Session {
            connected: false,
            accounts: Vec::new(),
            chain_id,
            bridge,
            key,
            client_id: Topic::new(),
            client_meta,
            peer_id: None,
            peer_meta: None,
            handshake_topic,
        }
    }
}
