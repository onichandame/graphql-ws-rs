use tokio::net::TcpStream;

use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, http::HeaderValue},
    MaybeTlsStream, WebSocketStream,
};

use crate::error::Error;

pub type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub async fn connect_ws(url: &str) -> Result<Connection, Error> {
    let mut request = url.into_client_request()?;
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws")?,
    );
    let (stream, _) = connect_async(request).await?;
    Ok(stream)
}
