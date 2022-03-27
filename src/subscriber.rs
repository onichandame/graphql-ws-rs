use serde_json::Value;
use tokio::sync::mpsc;

use crate::{error::Error, payload::SubscribePayload, response::Response};

pub type Subscription = mpsc::Receiver<Response>;

pub type SubscribeChannel = mpsc::Sender<(SubscribePayload, mpsc::Sender<Response>)>;

pub struct Subscriber {
    subscribe_channel: SubscribeChannel,
}

impl Subscriber {
    pub fn new(subscribe_channel: SubscribeChannel) -> Self {
        Self { subscribe_channel }
    }
    pub async fn subscribe(
        &self,
        query: &str,
        operation_name: Option<String>,
        variables: Option<Value>,
        extensions: Option<Value>,
    ) -> Result<Subscription, Error> {
        let (sender, receiver) = mpsc::channel(8);
        let subscribe_channel = self.subscribe_channel.clone();
        subscribe_channel
            .send((
                SubscribePayload {
                    query: query.to_owned(),
                    operation_name,
                    variables,
                    extensions,
                },
                sender.clone(),
            ))
            .await?;
        Ok(receiver)
    }
}
