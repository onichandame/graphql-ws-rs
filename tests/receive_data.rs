use std::error::Error as StdError;

use fixture::get_server;
use serde::Deserialize;
use tokio_graphql_ws::{Client, Response};

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn receive_data() -> Result<(), Error> {
    let (server_addr, server) = get_server().await?;
    let server = tokio::spawn(server);
    let client = Client::new().set_url(&format!("ws://{}", &server_addr.to_string()));
    let (connection, subscriber) = client.try_connect().await?;
    let client = tokio::spawn(connection);
    let mut receiver = subscriber
        .subscribe("query {greet(name:\"Bob\")}", None, None, None)
        .await?;
    let data = receiver.recv().await.ok_or("err")?;
    #[derive(Deserialize)]
    struct Data {
        greet: String,
    }
    match data {
        Response::Normal(data) => {
            let data = serde_json::from_value::<Data>(data.data)?;
            assert_eq!("hello Bob", data.greet);
        }
        Response::Error(_) => return Err("failed to receive data".into()),
    };
    client.abort();
    server.abort();
    Ok(())
}
