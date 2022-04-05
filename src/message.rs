use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::{self};
use uuid::Uuid;

use crate::{error::Error, request::Request};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    ConnectionInit,
    ConnectionAck,
    Ping,
    Pong,
    Subscribe,
    Next,
    Error,
    Complete,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

/// public api
impl Message {
    pub fn connection_init<T: Serialize>(payload: Option<T>) -> Result<Self, Error> {
        Ok(Self {
            type_: MessageType::ConnectionInit,
            id: None,
            payload: payload.map(|v| serde_json::to_value(&v).unwrap()),
        })
    }

    pub fn subscribe<TVariables: Serialize, TExtensions: Serialize>(
        payload: &Request<TVariables, TExtensions>,
    ) -> Result<Self, Error> {
        Ok(Self {
            type_: MessageType::Subscribe,
            id: Some(Uuid::new_v4().to_hyphenated().to_string()),
            payload: Some(serde_json::to_value(payload)?),
        })
    }

    pub fn pong<T: Serialize>(payload: Option<T>) -> Result<Self, Error> {
        Ok(Self {
            type_: MessageType::Pong,
            id: None,
            payload: payload.map(|v| serde_json::to_value(&v).unwrap()),
        })
    }

    pub fn from_ws_msg(msg: &tungstenite::Message) -> Result<Message, Error> {
        if let tungstenite::Message::Text(msg) = msg {
            Ok(serde_json::from_str(msg)?)
        } else {
            Err(format!(
                "message type not valid! only text messages are expected {}",
                msg
            )
            .into())
        }
    }

    pub fn into_ws_msg(&self) -> Result<tungstenite::Message, Error> {
        Ok(tungstenite::Message::Text(serde_json::to_string(self)?))
    }
}
