use async_trait::async_trait;
use serde_json::Value;

use crate::error::Error;

#[async_trait]
pub trait ClientActor: Send + Sync + Clone {
    async fn connection_init(&self) -> Result<Option<Value>, Error> {
        Ok(None)
    }
    async fn on_connection_ack(&self, _: Value) -> Result<(), Error> {
        Ok(())
    }
    async fn ping(&self) -> Result<Option<Value>, Error> {
        Ok(None)
    }
    async fn on_ping(&self, _: Option<Value>) -> Result<Option<Value>, Error> {
        Ok(None)
    }
    async fn on_pong(&self, _: Option<Value>) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct DefaultClientActor {}
#[async_trait]
impl ClientActor for DefaultClientActor {}
