use std::{error::Error, net::SocketAddr, time::Duration};

use async_graphql::{Context, Data, Object, Schema, Subscription};
use async_graphql_warp::{graphql_protocol, GraphQLWebSocket};
use futures_util::{stream, Future, Stream};
use serde::Deserialize;
use tokio::time;
use warp::{ws::Ws, Filter};

type Name = String;

pub async fn get_server(
) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), Box<dyn Error + Send + Sync>> {
    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .finish();
    let route = warp::ws()
        .and(graphql_protocol())
        .and(warp::any().map(move || schema.clone()))
        .map(move |ws: Ws, protocol, schema| {
            let reply = ws.on_upgrade(move |sock| {
                GraphQLWebSocket::new(sock, schema, protocol)
                    .on_connection_init(|v| async {
                        let mut data = Data::default();
                        #[derive(Deserialize)]
                        struct InitPayload {
                            name: Name,
                        }
                        if let Ok(init_payload) = serde_json::from_value::<InitPayload>(v) {
                            data.insert(init_payload.name);
                        }
                        Ok(data)
                    })
                    .serve()
            });
            warp::reply::with_header(
                reply,
                "Sec-Websocket-Protocol",
                protocol.sec_websocket_protocol(),
            )
        });
    let res = warp::serve(route).try_bind_ephemeral(([127, 0, 0, 1], 0))?;
    Ok(res)
}

#[derive(Default)]
struct Query {}
#[derive(Default)]
struct Mutation {}
#[derive(Default)]
struct Subscription {}

#[Object]
impl Query {
    async fn greet(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
    ) -> async_graphql::Result<String> {
        let name = match name {
            Some(name) => name,
            None => ctx.data::<Name>()?.to_owned(),
        };
        Ok(format!("hello {}", &name))
    }
    async fn error(&self) -> async_graphql::Result<bool> {
        Err("error".into())
    }
}

#[Object]
impl Mutation {
    async fn break_in(&self, name: String) -> async_graphql::Result<String> {
        Ok(format!("damn {}", name))
    }
}

#[Subscription]
impl Subscription {
    async fn ticker(&self, initial: Option<i32>) -> impl Stream<Item = i32> {
        let value = match initial {
            Some(v) => v,
            None => 0,
        };
        let interval = time::interval(Duration::from_millis(100));
        stream::unfold((interval, value), |(mut interval, mut value)| async move {
            interval.tick().await;
            let current = value.clone();
            value += 1;
            Some((current, (interval, value)))
        })
    }
}
