use futures_util::StreamExt;
use std::error::Error as StdError;
use tokio_graphql_ws::{Request, Subscriber};

use crate::fixture::{get_server_client, DefaultClient, Ticker};

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn subscription() -> Result<(), Error> {
    let (server, client) = get_server_client(DefaultClient::new).await?;
    let server = tokio::spawn(server);
    let mut receiver = client
        .subscribe(&Request::<(), ()> {
            query: "subscription {ticker(initial:0)}".to_owned(),
            ..Default::default()
        })
        .await?;
    for i in 0..10 {
        let data = receiver.next().await.ok_or("err")??;
        let data = serde_json::from_value::<Ticker>(data.data)?;
        assert_eq!(i, data.ticker);
    }
    server.abort();
    Ok(())
}
