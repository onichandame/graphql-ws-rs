use std::error::Error as StdError;

use async_trait::async_trait;
use fixture::get_server;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_graphql_ws::{Client, ClientActor, Response};

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn connection_init() -> Result<(), Error> {
    let (server_addr, server) = get_server().await?;
    let server = tokio::spawn(server);
    let client = Client::new()
        .set_url(&format!("ws://{}", &server_addr.to_string()))
        .set_actors(Actor::default());
    let (connection, subscriber) = client.try_connect().await?;
    let client = tokio::spawn(connection);
    let mut receiver = subscriber
        .subscribe("query {greet}", None, None, None)
        .await?;
    let data = receiver.recv().await.ok_or("err")?;
    #[derive(Deserialize)]
    struct Data {
        greet: String,
    }
    match data {
        Response::Normal(data) => {
            let data = serde_json::from_value::<Data>(data.data)?;
            assert_eq!("hello Alice", data.greet);
        }
        Response::Error(_) => return Err("failed to receive data".into()),
    };
    client.abort();
    server.abort();
    Ok(())
}

#[derive(Clone, Default)]
struct Actor {}

#[async_trait]
impl ClientActor for Actor {
    async fn connection_init(&self) -> Result<Option<Value>, Error> {
        Ok(Some(serde_json::to_value(InitPayload {
            name: "Alice".to_owned(),
        })?))
    }
}

#[derive(Serialize)]
struct InitPayload {
    name: String,
}
