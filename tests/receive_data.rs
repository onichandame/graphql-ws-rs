use std::error::Error as StdError;

use fixture::{get_server_client, DefaultClient, Greet};
use futures_util::StreamExt;
use tokio_graphql_ws::{Request, Subscriber};

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn receive_data() -> Result<(), Error> {
    let (server, client) = get_server_client(DefaultClient::new).await?;
    let server = tokio::spawn(server);
    let data = client
        .subscribe(&Request::<(), ()> {
            query: "query {greet(name:\"Bob\")}".to_owned(),
            ..Default::default()
        })
        .await?
        .next()
        .await
        .ok_or("err")??;
    assert_eq!(
        "hello Bob",
        serde_json::from_value::<Greet>(data.data)?.greet
    );
    server.abort();
    Ok(())
}
