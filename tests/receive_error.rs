use std::error::Error as StdError;

use fixture::get_server;
use tokio_graphql_ws::Client;

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn receive_error() -> Result<(), Error> {
    let (server_addr, server) = get_server().await?;
    let server = tokio::spawn(server);
    let client = Client::new().set_url(&format!("ws://{}", &server_addr.to_string()));
    let (connection, subscriber) = client.try_connect().await?;
    let client = tokio::spawn(connection);
    let mut receiver = subscriber
        .subscribe("query {error}", None, None, None)
        .await?;
    let data = receiver.recv().await.ok_or("err")??;
    assert!(data.errors.is_some());
    client.abort();
    server.abort();
    Ok(())
}
