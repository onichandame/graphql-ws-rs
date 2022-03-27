use std::{collections::HashMap, time::Duration};

use futures_util::{Future, SinkExt, StreamExt, TryStreamExt};
use tokio::{select, sync::mpsc, time::interval};

use crate::{
    client_actor::{ClientActor, DefaultClientActor},
    client_cfg::ClientConfig,
    connection::connect_ws,
    error::Error,
    message::{ws_close_message, Message, MessageType},
    subscriber::Subscriber,
};

#[derive(Default)]
pub struct Client<TActor: ClientActor = DefaultClientActor> {
    actors: TActor,
    cfg: ClientConfig,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<TActor: ClientActor> Client<TActor> {
    /// Try to establish an acknowledged connection to the server.
    ///
    /// Returns a future that starts the event loop and a subscriber used to talk to the loop
    pub async fn try_connect(&self) -> Result<(impl Future<Output = ()>, Subscriber), Error> {
        let mut stream = connect_ws(&self.cfg.url).await?;
        stream
            .send(Message::connection_init(self.actors.connection_init().await?)?.into_ws_msg()?)
            .await?;
        let msg = stream.try_next().await?.ok_or("connection closed")?;
        let msg = Message::from_ws_msg(&msg)?;
        if !matches!(msg.type_, MessageType::ConnectionAck) {
            return Err("connection ack not received".into());
        }
        if let Some(payload) = msg.payload {
            self.actors.on_connection_ack(payload).await?;
        }
        let (subscribe_channel, mut subscribe_receiver) = mpsc::channel(5);
        let connection = Subscriber::new(subscribe_channel);
        let cfg = self.cfg.clone();
        let actors = self.actors.clone();
        let fut = async move {
            let (ping_sender, mut ping_receiver) = mpsc::channel(1);
            if cfg.ping_interval > Duration::from_millis(0) {
                let mut ping_interval = interval(cfg.ping_interval.clone());
                tokio::spawn(async move {
                    loop {
                        ping_interval.tick().await;
                        ping_sender.send(()).await.unwrap();
                    }
                });
            }
            let mut subscribers = HashMap::new();
            loop {
                select! {
                    _ = ping_receiver.recv() => {
                        stream.send(Message::ping(actors.ping().await.unwrap()).unwrap().into_ws_msg().unwrap()).await.unwrap();
                    },
                    msg = subscribe_receiver.recv() => {
                        if let Some((payload,sender)) = msg {
                            let msg = Message::subscribe(&payload).unwrap();
                            let id = msg.id.as_ref().ok_or("subscribe message must have id").unwrap();
                            subscribers.insert(id.to_owned(), sender);
                            stream.send(msg.into_ws_msg().unwrap()).await.unwrap();
                        } else {
                            break
                        }
                    },
                    msg = stream.next() => {
                        if let Some(Ok(msg)) = msg {
                            let msg = Message::from_ws_msg(&msg).unwrap();
                            match msg.type_{
                                MessageType::Next => {
                                    let id=msg.id.ok_or("next message missing id").unwrap();
                                    let sender=subscribers.get(&id).ok_or("cannot next on a closed subscription").unwrap();
                                    let payload=msg.payload.ok_or("no payload received from next message").unwrap();
                                    if sender.send(Ok(serde_json::from_value(payload).unwrap())).await.is_err() {
                                            subscribers.remove(&id);
                                    }
                                },
                                MessageType::Error => {
                                    let id=msg.id.ok_or("error message missing id").unwrap();
                                    let sender=subscribers.get(&id).ok_or("cannot error on a closed subscription").unwrap();
                                    let payload=msg.payload.ok_or("no payload received from error message").unwrap();
                                    if sender.send(Err(serde_json::from_value(payload).unwrap())).await.is_err(){
                                        subscribers.remove(&id);
                                    }
                                },
                                MessageType::Complete => {
                                    let id=msg.id.ok_or("complete message missing id").unwrap();
                                    subscribers.remove(&id);
                                },
                                MessageType::Ping => {
                                    let msg=Message::pong(actors.on_ping(msg.payload).await.unwrap()).unwrap();
                                    stream.send(msg.into_ws_msg().unwrap()).await.unwrap();
                                },
                                MessageType::Pong=>{
                                    actors.on_pong(msg.payload).await.unwrap();
                                },
                                _other => {
                                    stream.send(ws_close_message(4400, "message type invalid")).await.unwrap();
                                    break
                                },
                            }
                        } else {
                            break
                        }
                    }
                };
            }
        };
        Ok((fut, connection))
    }

    pub fn set_actors<TActor2: ClientActor>(self, actors: TActor2) -> Client<TActor2> {
        Client {
            actors,
            cfg: self.cfg,
        }
    }

    /// 0: disable
    pub fn set_ping_interval(self, interval: Duration) -> Self {
        Self {
            cfg: ClientConfig {
                ping_interval: interval,
                ..self.cfg
            },
            ..self
        }
    }

    pub fn set_url(self, url: &str) -> Self {
        Self {
            cfg: ClientConfig {
                url: url.to_owned(),
                ..self.cfg
            },
            ..self
        }
    }
}
