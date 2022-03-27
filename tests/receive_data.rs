use std::error::Error as StdError;

use fixture::get_server;
use serde::Deserialize;
use tokio_graphql_ws::Client;

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
    #[derive(Deserialize)]
    struct Data {
        greet: String,
    }
    let data = receiver.recv().await.ok_or("err")??;
    assert_eq!(
        "hello Bob",
        serde_json::from_value::<Data>(data.data)?.greet
    );
    client.abort();
    server.abort();
    Ok(())
}
