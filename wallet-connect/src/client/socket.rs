//! Copyright (c) 2021 HIHAHEHO Studio (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use std::sync::atomic::Ordering;

use ethers::prelude::Address;
use futures::{future, SinkExt, TryStreamExt};
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        oneshot,
    },
    task::JoinHandle,
};
use url::Url;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use super::core::SharedContext;
use crate::{
    crypto::Key,
    protocol::{SocketMessage, SocketMessageKind, Topic},
    Request, Response,
};
use eyre::{eyre, Context};

/// This structure holds the websocket connection
#[derive(Debug)]
pub struct Socket {
    /// queue for messages to be sent to the bridge server
    /// TODO: make it bounded?
    sender: UnboundedSender<(Option<u64>, Vec<u8>)>,
    /// the handle of the task that writes on the websocket connection
    _write_handle: JoinHandle<()>,
    /// the handle of the task that reads on the websocket connection
    _read_handle: JoinHandle<()>,
}

/// A helper wrapper for processing the received messages
pub struct MessageHandler {
    // the WalletConnect client state
    pub context: SharedContext,
}

impl MessageHandler {
    /// Processes the decrypted message and returns the message to be sent back (if any)
    pub fn handle(&self, _: Topic, payload: Vec<u8>) -> Option<Vec<u8>> {
        // FIXME: one can also receive session upgrades, not only responses to previous requests
        let resp: Response<serde_json::Value> = serde_json::from_slice(&payload).ok()?;
        let (_id, sender) = self.context.0.pending_requests.remove(&resp.id)?;
        let _ = sender.send(resp.data.into_value().ok()?);
        None
    }
}

/// returns a topic and the decrypted payload
fn check_socket_msg(mmsg: Vec<u8>, key: &Key) -> Option<(Topic, Vec<u8>)> {
    match serde_json::from_slice::<SocketMessage>(&mmsg) {
        Ok(msg) if !matches!(msg.kind, SocketMessageKind::Sub) && msg.payload.is_some() => {
            let topic = msg.topic;
            // unwrap is safe -- it's checked above in the match clause if it's Some
            let payload = msg.payload.unwrap();
            if let Ok(decrypted) = key.open(&payload) {
                Some((topic, decrypted))
            } else {
                None
            }
        }
        _ => None,
    }
}

impl Socket {
    fn send_socket_msg(
        &self,
        context: &SharedContext,
        id: u64,
        msg: SocketMessage,
    ) -> eyre::Result<()> {
        if let Err(_e) = self.sender.send((Some(id), serde_json::to_vec(&msg)?)) {
            // not to let the requester to wait forever
            const ERROR_MSG: &str = "\"Failed to send message to the queue\"";
            if let Some((_id, sender)) = context.0.pending_requests.remove(&id) {
                let _ = sender.send(serde_json::json!(ERROR_MSG));
            }
            Err(eyre!(ERROR_MSG))
        } else {
            Ok(())
        }
    }

    /// sends a json-rpc request (encrypted for the wallet) via the bridge server
    /// and awaits the response
    pub async fn json_rpc_request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        id: u64,
        method: &str,
        params: T,
        context: &SharedContext,
    ) -> eyre::Result<R> {
        let (tx, rx) = oneshot::channel();
        context.0.pending_requests.insert(id, tx);
        let session = context.0.session.lock().await;
        let topic = session
            .peer_id
            .clone()
            .unwrap_or_else(|| session.handshake_topic.clone());
        let key = &session.key;
        let message = SocketMessage {
            kind: SocketMessageKind::Pub,
            topic,
            payload: Some(key.seal(serde_json::to_string(&Request::new(id, method, params))?)),
            silent: true,
        };
        drop(session);
        self.send_socket_msg(context, id, message)?;
        let response = rx.await?;
        serde_json::from_value(response).wrap_err("failed to parse response")
    }

    /// attempts to create a session with the external wallet,
    /// and returns the wallet's addresses and chain ID (if successful)
    pub async fn create_session(
        &mut self,
        id: u64,
        context: &mut SharedContext,
    ) -> eyre::Result<(Vec<Address>, u64)> {
        let session = context.0.session.lock().await;
        if session.connected {
            return Err(eyre!("Session already connected"));
        }
        if context
            .0
            .session_pending
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(eyre!("Session already pending"));
        }
        let (tx, rx) = oneshot::channel();
        context.0.pending_requests.insert(id, tx);
        let topic = session.handshake_topic.clone();
        let key = &session.key;
        let session_req = session.request();
        let message = SocketMessage {
            kind: SocketMessageKind::Pub,
            topic,
            payload: Some(key.seal(serde_json::to_string(&Request::new(
                id,
                "wc_sessionRequest",
                vec![session_req],
            ))?)),
            silent: true,
        };
        drop(session);
        self.send_socket_msg(context, id, message)?;
        let response = rx.await?;
        let session_params = serde_json::from_value(response)?;
        let mut session = context.0.session.lock().await;
        session.apply(session_params);
        context.0.session_pending.store(false, Ordering::SeqCst);
        Ok((
            session.accounts.clone(),
            session.chain_id.unwrap_or_default(),
        ))
    }

    /// sends a subscription for the given topic
    pub async fn subscribe(&mut self, topic: Topic) -> eyre::Result<()> {
        let msg = SocketMessage {
            kind: SocketMessageKind::Sub,
            topic,
            payload: None,
            silent: true,
        };
        let payload = serde_json::to_vec(&msg)?;
        self.sender.send((None, payload))?;
        Ok(())
    }

    /// connects to the bridge server via a websocket
    /// and starts the send/receive tasks
    /// TODO: handle reconnections?
    pub async fn connect(url: Url, key: Key, handler: MessageHandler) -> eyre::Result<Self> {
        let (mut tx, rx) = connect(url).await?.split();
        let (sender, mut receiver) = unbounded_channel::<(Option<u64>, Vec<u8>)>();
        let sender_out = sender.clone();
        let context = handler.context.clone();

        // a task for reading from the websocket connection, decrypting the data
        // and sending them as responses to the previous requests by the message handler
        let reader = tokio::spawn(async move {
            let _ = rx
                .try_filter_map(|mmsg| future::ok(check_socket_msg(mmsg, &key)))
                .try_for_each(|(topic, decrypted)| {
                    if let Some(resp) = handler.handle(topic, decrypted) {
                        let _ = sender.send((None, resp));
                    }
                    future::ok(())
                })
                .await;
        });
        // a task for sending the messages to the bridge server
        let writer = tokio::spawn(async move {
            while let Some((mid, x)) = receiver.recv().await {
                if let (Err(_), Some(id)) = (tx.send(x).await, mid) {
                    // not to let the requester to wait forever
                    const ERROR_MSG: &str = "\"Failed to send message to the bridge server\"";
                    if let Some((_id, sender)) = context.0.pending_requests.remove(&id) {
                        let _ = sender.send(serde_json::json!(ERROR_MSG));
                    }
                }
            }
        });
        Ok(Self {
            sender: sender_out,
            _write_handle: writer,
            _read_handle: reader,
        })
    }
}

/// a wrapper type that holds the split websocket connection
pub struct WebSocketClient<Tx, Rx> {
    tx: Tx,
    rx: Rx,
}

impl<Tx, Rx> WebSocketClient<Tx, Rx> {
    /// get the writer and reader streams
    pub fn split(self) -> (Tx, Rx) {
        (self.tx, self.rx)
    }
}

/// error type from the websocket connection
/// TODO: Refine
#[non_exhaustive]
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SinkError {
    #[error("send failed {0}")]
    Send(String),
}

/// the native implementation for websocket connections
#[cfg(not(target_arch = "wasm32"))]
mod native {

    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message;
    use url::Url;

    use super::{SinkError, WebSocketClient};

    use eyre::Result;
    use futures::future::{ready, Ready};
    use futures::{prelude::*, Sink, Stream};

    type Bytes = Vec<u8>;

    /// connects using `tokio_tungstenite` to the bridge server
    /// and returns the writer and reader streams wrapped in a struct
    pub async fn connect<'a>(
        addr: Url,
    ) -> Result<
        WebSocketClient<
            impl Sink<Bytes, Error = SinkError> + Send + Sync + Unpin,
            impl Stream<Item = Result<Bytes>> + Send + Sync + Unpin,
        >,
    > {
        let (stream, _response) = connect_async(addr.as_ref()).await?;
        let (tx, rx) = stream.split();
        let rx = rx.map(|message| -> Result<Bytes> { Ok(message?.into_data()) });
        let tx = tx
            .sink_map_err(|err| SinkError::Send(err.to_string()))
            .with(|bytes: Bytes| -> Ready<Result<Message, SinkError>> {
                ready(Ok(Message::binary(bytes)))
            });

        Ok(WebSocketClient { tx, rx })
    }
}

/// the in-browser implementation for websocket connections
/// TODO: is the wasm implementation needed?
/// The WalletConnect has official client NPM packages: https://docs.walletconnect.com/quick-start/dapps/client
/// so assuming this, one will want to use the official JS package and no need to compile this walletconnect client to wasm?
#[cfg(target_arch = "wasm32")]
mod wasm {
    use url::Url;
    use ws_stream_wasm::{WsMessage, WsMeta};

    use crate::{SinkError, WebSocketClient};
    use eyre::Result;
    use futures::future::{ready, Ready};
    use futures::{prelude::*, Sink, Stream};

    type Bytes = Vec<u8>;

    pub async fn connect<'a>(
        addr: Url,
    ) -> Result<
        WebSocketClient<
            impl Sink<Bytes, Error = SinkError> + Send + Sync + Unpin,
            impl Stream<Item = Result<Bytes>> + Send + Sync + Unpin,
        >,
    > {
        let (_, wsio) = WsMeta::connect(addr.as_ref(), None).await?;
        let (tx, rx) = wsio.split();
        let rx = rx.map(|message| -> Result<Bytes> { Ok(message.into()) });
        let tx = tx
            .sink_map_err(|err| SinkError::Send(err.to_string()))
            .with(|bytes: Bytes| -> Ready<Result<WsMessage, SinkError>> {
                ready(Ok(WsMessage::Binary(bytes)))
            });

        Ok(WebSocketClient { tx, rx })
    }
}
