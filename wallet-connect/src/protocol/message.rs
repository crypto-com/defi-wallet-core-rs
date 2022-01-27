//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use super::Topic;
use crate::serialization;
use serde::{Deserialize, Serialize};

/// the outer message type send to or received from the bridge server
/// via websockets
/// https://docs.walletconnect.com/tech-spec#websocket-messages
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SocketMessage {
    /// the topic of the message (UUID to denote the peer or handshake)
    pub topic: Topic,
    #[serde(rename = "type")]
    /// whether publishing or subscribing
    pub kind: SocketMessageKind,
    /// the encrypted payload (if any)
    #[serde(with = "serialization::jsonstring")]
    pub payload: Option<EncryptionPayload>,
    /// a new field present in bridge serve messages
    /// (or shall serde allow unknown fields?)
    #[serde(default)]
    pub silent: bool,
}

/// whether publishing or subscribing
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SocketMessageKind {
    /// Publish
    Pub,
    /// Subscribe
    Sub,
}

/// The encrypted payload -- the plaintext is usually a JSON-RPC request or response
/// https://docs.walletconnect.com/tech-spec#cryptography
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EncryptionPayload {
    /// the encrypted payload
    #[serde(with = "serialization::hexstring")]
    pub data: Vec<u8>,
    /// HMAC-SHA256 of the encrypted payload and nonce
    #[serde(with = "serialization::hexstring")]
    pub hmac: Vec<u8>,
    /// nonce
    #[serde(with = "serialization::hexstring")]
    pub iv: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn message_serialization() {
        let message = SocketMessage {
            topic: "de5682be-2a03-4b8e-866e-1e89dbca422b".parse().unwrap(),
            kind: SocketMessageKind::Pub,
            payload: Some(EncryptionPayload {
                data: vec![0x04, 0x2],
                hmac: vec![0x13, 0x37],
                iv: vec![0x00],
            }),
            silent: false,
        };
        let json = json!({
            "topic": "de5682be-2a03-4b8e-866e-1e89dbca422b",
            "type": "pub",
            "payload": "{\"data\":\"0402\",\"hmac\":\"1337\",\"iv\":\"00\"}",
            "silent": false,
        });

        assert_eq!(serde_json::to_value(&message).unwrap(), json);
        assert_eq!(
            serde_json::from_value::<SocketMessage>(json).unwrap(),
            message
        );
    }
}
