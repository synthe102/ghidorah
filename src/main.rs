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

use std::io::Read;

use flate2::bufread::GzDecoder;
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
    let (ws_stream, _) = connect_async_tls_with_config(
        "wss://foo-pinned.koko.localhost:3100/v1/outlet?node_id=93FD3924-526C-4B39-BF83-16CCF5F55D18&node_hostname=localhost&node_version=3.6.0.0",
        None,
        false,
        Option::from(connector),
    )
    .await
    .expect("Failed to connect");

    println!("Connected to server");
    let (mut write, mut read) = ws_stream.split();

    // Send info about the node
    write 
        .send(Message::Text("{\"labels\":{\"koko-wd\":\"true\",\"node\":\"1\",\"version\":\"3.6.0.0\"},\"plugins\":[{\"name\":\"azure-functions\",\"version\":\"3.6.0.0\"},{\"name\":\"ip-restriction\",\"version\":\"3.6.0.0\"},{\"name\":\"request-termination\",\"version\":\"3.6.0.0\"},{\"name\":\"route-transformer-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"syslog\",\"version\":\"3.6.0.0\"},{\"name\":\"tls-metadata-headers\",\"version\":\"3.6.0.0\"},{\"name\":\"correlation-id\",\"version\":\"3.6.0.0\"},{\"name\":\"openid-connect\",\"version\":\"3.6.0.0\"},{\"name\":\"statsd-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"jwe-decrypt\",\"version\":\"3.6.0.0\"},{\"name\":\"opa\",\"version\":\"3.6.0.0\"},{\"name\":\"request-transformer\",\"version\":\"3.6.0.0\"},{\"name\":\"request-transformer-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"response-transformer-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"statsd\",\"version\":\"3.6.0.0\"},{\"name\":\"vault-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"grpc-web\",\"version\":\"3.6.0.0\"},{\"name\":\"jwt\",\"version\":\"3.6.0.0\"},{\"name\":\"jwt-signer\",\"version\":\"3.6.0.0\"},{\"name\":\"oauth2-introspection\",\"version\":\"3.6.0.0\"},{\"name\":\"websocket-validator\",\"version\":\"3.6.0.0\"},{\"name\":\"upstream-timeout\",\"version\":\"3.6.0.0\"},{\"name\":\"aws-lambda\",\"version\":\"3.6.0.0\"},{\"name\":\"bot-detection\",\"version\":\"3.6.0.0\"},{\"name\":\"canary\",\"version\":\"3.6.0.0\"},{\"name\":\"cors\",\"version\":\"3.6.0.0\"},{\"name\":\"datadog\",\"version\":\"3.6.0.0\"},{\"name\":\"rate-limiting\",\"version\":\"3.6.0.0\"},{\"name\":\"request-size-limiting\",\"version\":\"3.6.0.0\"},{\"name\":\"acme\",\"version\":\"3.6.0.0\"},{\"name\":\"http-log\",\"version\":\"3.6.0.0\"},{\"name\":\"jq\",\"version\":\"3.6.0.0\"},{\"name\":\"konnect-application-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"post-function\",\"version\":\"3.6.0.0\"},{\"name\":\"pre-function\",\"version\":\"3.6.0.0\"},{\"name\":\"tls-handshake-modifier\",\"version\":\"3.6.0.0\"},{\"name\":\"application-registration\",\"version\":\"3.6.0.0\"},{\"name\":\"graphql-rate-limiting-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"opentelemetry\",\"version\":\"3.6.0.0\"},{\"name\":\"proxy-cache-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"request-validator\",\"version\":\"3.6.0.0\"},{\"name\":\"response-ratelimiting\",\"version\":\"3.6.0.0\"},{\"name\":\"session\",\"version\":\"3.6.0.0\"},{\"name\":\"basic-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"saml\",\"version\":\"3.6.0.0\"},{\"name\":\"tcp-log\",\"version\":\"3.6.0.0\"},{\"name\":\"file-log\",\"version\":\"3.6.0.0\"},{\"name\":\"response-transformer\",\"version\":\"3.6.0.0\"},{\"name\":\"route-by-header\",\"version\":\"3.6.0.0\"},{\"name\":\"hmac-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"key-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"mtls-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"rate-limiting-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"zipkin\",\"version\":\"3.6.0.0\"},{\"name\":\"oauth2\",\"version\":\"3.6.0.0\"},{\"name\":\"udp-log\",\"version\":\"3.6.0.0\"},{\"name\":\"loggly\",\"version\":\"3.6.0.0\"},{\"name\":\"exit-transformer\",\"version\":\"3.6.0.0\"},{\"name\":\"key-auth-enc\",\"version\":\"3.6.0.0\"},{\"name\":\"ldap-auth\",\"version\":\"3.6.0.0\"},{\"name\":\"mocking\",\"version\":\"3.6.0.0\"},{\"name\":\"proxy-cache\",\"version\":\"3.6.0.0\"},{\"name\":\"kafka-upstream\",\"version\":\"3.6.0.0\"},{\"name\":\"ldap-auth-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"xml-threat-protection\",\"version\":\"3.6.0.0\"},{\"name\":\"degraphql\",\"version\":\"3.6.0.0\"},{\"name\":\"forward-proxy\",\"version\":\"3.6.0.0\"},{\"name\":\"kafka-log\",\"version\":\"3.6.0.0\"},{\"name\":\"oas-validation\",\"version\":\"3.6.0.0\"},{\"name\":\"acl\",\"version\":\"3.6.0.0\"},{\"name\":\"graphql-proxy-cache-advanced\",\"version\":\"3.6.0.0\"},{\"name\":\"grpc-gateway\",\"version\":\"3.6.0.0\"},{\"name\":\"prometheus\",\"version\":\"3.6.0.0\"},{\"name\":\"websocket-size-limit\",\"version\":\"3.6.0.0\"}],\"type\":\"basic_info\"}".into()))
        .await
        .unwrap();
    let _ = write.flush();

    tokio::spawn(async move {
        loop {
            println!("Sending ping");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            write.send(Message::Ping("0".repeat(32).into_bytes())).await.unwrap();
        }
    });

    // Receive messages from the server
    while let Some(Ok(message)) = read.next().await {
        match message {
            Message::Text(text) => {
                println!("Received text message: {}", text);
            }
            Message::Binary(data) => {
                let mut d = GzDecoder::new(&data[..]);
                let mut s = String::new();
                d.read_to_string(&mut s).unwrap();
                println!("Received binary message, that was decoded into: {:?}", s);
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

    //TODO: Parse the kong config
    //TODO: Configure Pingora with the kong config
    //TODO: Start the Pingora server
    //TODO: hot reload the Pingora server when the kong config changes

    println!("Connection closed");
}
