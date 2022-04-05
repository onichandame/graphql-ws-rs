use std::error::Error as StdError;

use fixture::{get_server_client, Greet, NamedClient};
use futures_util::StreamExt;
use tokio_graphql_ws::{Request, Subscriber};

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn connection_init() -> Result<(), Error> {
    let (server, client) = get_server_client(NamedClient::new("Alice")).await?;
    let server = tokio::spawn(server);
    let data = client
        .subscribe(&Request::<(), ()> {
            query: "query {greet}".to_owned(),
            ..Default::default()
        })
        .await?
        .next()
        .await
        .ok_or("err")??;
    assert_eq!(
        "hello Alice",
        serde_json::from_value::<Greet>(data.data)?.greet
    );
    server.abort();
    Ok(())
}
