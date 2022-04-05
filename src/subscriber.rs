use async_trait::async_trait;
use futures_util::{stream::BoxStream, SinkExt, StreamExt, TryStreamExt};
use serde::Serialize;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        http::HeaderValue,
        protocol::{frame::coding::CloseCode, CloseFrame},
    },
};

use crate::{
    error::Error,
    message::{Message, MessageType},
    request::Request,
    response::Response,
    ClientTrait,
};

#[async_trait]
pub trait Subscriber: ClientTrait {
    fn url(&self) -> String;
    async fn subscribe<TVariables: Serialize + Sync, TExtensions: Serialize + Sync>(
        &self,
        payload: &Request<TVariables, TExtensions>,
    ) -> Result<BoxStream<'_, Response>, Error> {
        // setup ws connection
        let mut request = self.url().into_client_request()?;
        request.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_str("graphql-transport-ws")?,
        );
        let (mut stream, _) = connect_async(request).await?;
        // handle graphql-ws initialization
        stream
            .send(Message::connection_init(self.connection_init().await?)?.into_ws_msg()?)
            .await?;
        let msg = stream.try_next().await?.ok_or("connection closed")?;
        let msg = Message::from_ws_msg(&msg)?;
        if !matches!(msg.type_, MessageType::ConnectionAck) {
            return Err("connection ack not received".into());
        }
        if let Some(payload) = msg.payload {
            self.on_connection_ack(payload).await?;
        }
        // send request
        let msg = Message::subscribe(payload)?;
        stream.send(msg.into_ws_msg().unwrap()).await?;
        // process response stream
        let res =
            futures::stream::unfold((stream, self.clone()), |(mut stream, actors)| async move {
                match stream.next().await {
                    Some(v) => {
                        let msg = v.unwrap();
                        let msg = Message::from_ws_msg(&msg).unwrap();
                        match msg.type_ {
                            MessageType::Next => {
                                let payload = msg
                                    .payload
                                    .ok_or("no payload received from next message")
                                    .unwrap();
                                Some((
                                    Some(Ok(serde_json::from_value(payload).unwrap())),
                                    (stream, actors),
                                ))
                            }
                            MessageType::Error => {
                                let payload = msg
                                    .payload
                                    .ok_or("no payload received from error message")
                                    .unwrap();
                                Some((
                                    Some(Err(serde_json::from_value(payload).unwrap())),
                                    (stream, actors),
                                ))
                            }
                            MessageType::Ping => {
                                let msg = Message::pong(actors.on_ping(msg.payload).await.unwrap())
                                    .unwrap();
                                stream.send(msg.into_ws_msg().unwrap()).await.unwrap();
                                Some((None, (stream, actors)))
                            }
                            MessageType::Pong => {
                                actors.on_pong(msg.payload).await.unwrap();
                                Some((None, (stream, actors)))
                            }
                            other => {
                                stream
                                    .close(Some(match other {
                                        MessageType::Complete => CloseFrame {
                                            code: CloseCode::Normal,
                                            reason: "Normal Closure".into(),
                                        },
                                        _other => CloseFrame {
                                            code: CloseCode::Reserved(4400),
                                            reason: "message type invalid".into(),
                                        },
                                    }))
                                    .await
                                    .unwrap();
                                None
                            }
                        }
                    }
                    None => None,
                }
            })
            .filter_map(|v| async move { v });
        Ok(Box::pin(res))
    }
}
