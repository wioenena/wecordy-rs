use futures_util::FutureExt;
use rust_socketio::{
    Event, Payload, TransportType,
    asynchronous::{Client, ClientBuilder},
};
use serde_json::json;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, instrument};
use wecordy_api::models;

mod connection_state;
mod error;
pub mod payloads;
pub use connection_state::ConnectionState;
pub use error::{Error, Result};

use crate::payloads::GatewayReceivePayload;

#[derive(Clone)]
pub struct WebSocket {
    pub(crate) token: Arc<str>,
    pub(crate) intents: Arc<Vec<String>>,
    pub(crate) ws_url: Arc<str>,
    pub(crate) user: Arc<Option<models::user::User>>,
    pub(crate) client: Arc<Mutex<Option<Client>>>,
    pub(crate) state: Arc<Mutex<ConnectionState>>,
}

impl WebSocket {
    pub fn new(token: Arc<str>, intents: Arc<Vec<String>>, ws_url: Arc<str>) -> Self {
        Self {
            token,
            intents,
            ws_url,
            user: Arc::new(None),
            client: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        if self.get_state().await != ConnectionState::Disconnected {
            return Err(Error::AlreadyConnected);
        }
        self.set_state(ConnectionState::Connecting).await;

        let socket = ClientBuilder::new(self.ws_url.as_ref())
            .on(Event::Connect, {
                let this = self.clone();
                move |_, socket: Client| {
                    let this = this.clone();
                    async move {
                        this.on_connect(socket)
                            .await
                            .inspect_err(|e| todo!("{e:#?}"))
                            .unwrap();
                    }
                    .boxed()
                }
            })
            .on(Event::Error, {
                let this = self.clone();
                move |err, _| {
                    let this = this.clone();
                    async move {
                        this.on_error(err)
                            .await
                            .inspect_err(|e| todo!("{e:#?}"))
                            .unwrap();
                    }
                    .boxed()
                }
            })
            .on("message", {
                let this = self.clone();
                move |payload, _| {
                    let mut this = this.clone();
                    async move {
                        this.on_message(payload)
                            .await
                            .inspect_err(|e| todo!("{e:#?}"))
                            .unwrap();
                    }
                    .boxed()
                }
            })
            .transport_type(TransportType::Websocket)
            .connect()
            .await?;

        let mut client_guard = self.client.lock().await;
        *client_guard = Some(socket);
        drop(client_guard);
        Ok(())
    }

    #[instrument(skip_all)]
    async fn on_connect(&self, client: Client) -> Result<()> {
        self.send(
            &client,
            "authentication",
            json!({
                "token": format!("Bot {}", self.token.as_ref()),
                "intents": Arc::clone(&self.intents),
            }),
        )
        .await?;

        Ok(())
    }

    #[instrument(skip_all)]
    async fn on_error(&self, err: Payload) -> Result<()> {
        error!("SocketIO connection error: {:#?}", err);
        Ok(())
    }

    #[instrument(skip_all)]
    async fn on_message(&mut self, payload: Payload) -> Result<()> {
        match payload {
            Payload::Text(data_vec) => {
                let first_data = data_vec.into_iter().next().ok_or_else(|| todo!()).unwrap();
                let event: GatewayReceivePayload = serde_json::from_value(first_data)?;
                if let GatewayReceivePayload::Me(me) = event {
                    self.user = Arc::new(Some(*me));
                }
            }
            _ => todo!(),
        }
        Ok(())
    }

    #[instrument(skip_all, fields(event=%event))]
    async fn send(&self, client: &Client, event: &str, data: serde_json::Value) -> Result<()> {
        client.emit(event, data).await?;
        debug!("sent event: {}", event);
        Ok(())
    }

    async fn set_state(&self, state: ConnectionState) {
        let mut guard = self.state.lock().await;
        *guard = state;
    }

    pub async fn get_state(&self) -> ConnectionState {
        *self.state.lock().await
    }
}

impl Debug for WebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocket")
            .field("token", &self.token)
            .field("ws_url", &self.ws_url)
            .field("state", &self.state)
            .finish()
    }
}
