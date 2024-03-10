//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use futures_util::{SinkExt, StreamExt};
use tokio_native_tls::native_tls::TlsConnector;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message, Connector};

#[tokio::main]
async fn main() {
    let identity = tokio_native_tls::native_tls::Identity::from_pkcs8(
        &std::fs::read("./foo/cluster.crt").unwrap(),
        &std::fs::read("./foo/cluster.key").unwrap(),
    )
    .unwrap();
    let connector = TlsConnector::builder()
        .identity(identity)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector = Connector::NativeTls(connector);
    // Connect to the WebSocket server
    let (mut ws_stream, _) = connect_async_tls_with_config(
        "wss://foo-pinned.koko.localhost:3100/v1/outlet?node_id=93FD3924-526C-4B39-BF83-16CCF5F55D18&node_hostname=localhost&node_version=3.6.0.0",
        None,
        false,
        Option::from(connector),
    )
    .await
    .expect("Failed to connect");

    println!("Connected to server");

    ws_stream
        .send(Message::Text("Hello server".into()))
        .await
        .unwrap();
    // Send a message to the server

    println!("Message sent to server");

    // Receive messages from the server
    while let Some(Ok(message)) = ws_stream.next().await {
        match message {
            Message::Text(text) => {
                println!("Received text message: {}", text);
            }
            Message::Binary(data) => {
                println!("Received binary message: {:?}", data);
            }
            Message::Ping(_) => {
                println!("Received ping");
            }
            Message::Pong(_) => {
                println!("Received pong");
            }
            Message::Frame(_) => {
                println!("Received a frame");
            }
            Message::Close(_) => {
                println!("Received close message");
                break;
            }
        }
    }

    println!("Connection closed");
}
