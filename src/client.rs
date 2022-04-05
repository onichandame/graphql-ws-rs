use async_trait::async_trait;
use serde_json::Value;

use crate::error::Error;

#[async_trait]
pub trait ClientTrait: Clone + Send + Sync {
    /// returns payload to send to server for connection initialization
    async fn connection_init(&self) -> Result<Option<Value>, Error> {
        Ok(None)
    }
    /// do something about the returned payload from server for connection acknowledgement
    async fn on_connection_ack(&self, _: Value) -> Result<(), Error> {
        Ok(())
    }
    /// returns payload to pong the server for every ping received
    async fn on_ping(&self, _: Option<Value>) -> Result<Option<Value>, Error> {
        Ok(None)
    }
    /// do something about the pong received from server
    async fn on_pong(&self, _: Option<Value>) -> Result<(), Error> {
        Ok(())
    }
}
