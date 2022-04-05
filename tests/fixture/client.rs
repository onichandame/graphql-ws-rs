use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_graphql_ws::{ClientTrait, Subscriber};

#[derive(Clone, ClientTrait, Subscriber)]
#[graphql_ws(url = "self.url.clone()")]
pub struct DefaultClient {
    pub url: String,
}

#[derive(Clone, Subscriber, Serialize)]
#[graphql_ws(url = "self.url.clone()")]
pub struct NamedClient {
    pub name: String,
    #[serde(skip)]
    pub url: String,
}

#[derive(Deserialize)]
pub struct Greet {
    pub greet: String,
}

#[derive(Deserialize)]
pub struct Ticker {
    pub ticker: i32,
}

impl DefaultClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

impl NamedClient {
    pub fn new(name: &str) -> impl Fn(&str) -> Self + '_ {
        |url| Self {
            name: name.to_owned(),
            url: url.to_owned(),
        }
    }
}

#[async_trait]
impl ClientTrait for NamedClient {
    async fn connection_init(&self) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        Ok(Some(serde_json::to_value(self.clone())?))
    }
}
