mod client;
mod server;

use std::error::Error;

use futures_util::Future;
use tokio_graphql_ws::Subscriber;

use self::server::get_server;

pub use client::*;

pub async fn get_server_client<Client: Subscriber, TNewClient: Fn(&str) -> Client>(
    new_client: TNewClient,
) -> Result<(impl Future<Output = ()> + 'static, Client), Box<dyn Error + Send + Sync>> {
    let (server_addr, server) = get_server().await?;
    let client = new_client(&format!("ws://{}", &server_addr.to_string()));
    Ok((server, client))
}
