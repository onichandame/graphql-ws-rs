use std::error::Error as StdError;

use futures_util::StreamExt;
use tokio_graphql_ws::{Request, Subscriber};

use crate::fixture::{get_server_client, DefaultClient};

mod fixture;

type Error = Box<dyn StdError + Send + Sync>;

#[tokio::test]
async fn receive_error() -> Result<(), Error> {
    let (server, client) = get_server_client(DefaultClient::new).await?;
    let server = tokio::spawn(server);
    let data = client
        .subscribe(&Request::<(), ()> {
            query: "".to_owned(),
            ..Default::default()
        })
        .await?
        .next()
        .await
        .ok_or("err")??;
    match data.errors {
        Some(v) => {
            assert!(v.len() > 0)
        }
        None => {
            panic!("no error is thrown")
        }
    }
    server.abort();
    Ok(())
}
