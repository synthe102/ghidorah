pub mod config;
pub mod node;

use flate2::bufread::GzDecoder;
use futures_util::{SinkExt, StreamExt};
use serde_derive::{Deserialize, Serialize};
use std::{io::Read, sync::Arc};
use tokio::sync::Mutex;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message, Connector};

#[derive(Debug, Serialize, Deserialize)]
pub struct Konglet {
    pub node_info: node::NodeInfo,
    pub node_version: String,
    pub node_id: uuid::Uuid,
    pub cp_url: String,
    pub cert: String,
    key: String,
}

impl Konglet {
    pub fn new(
        node_info: node::NodeInfo,
        node_version: String,
        cp_url: String,
        cert: String,
        key: String,
    ) -> Self {
        Konglet {
            node_info,
            node_version,
            node_id: uuid::Uuid::new_v4(),
            cp_url,
            cert,
            key,
        }
    }
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub async fn run(&self) {
        //  Create the identity for the TLS connection
        let cert = self.cert.clone();
        let key = self.key.clone();
        let identity = tokio_native_tls::native_tls::Identity::from_pkcs8(
            &std::fs::read(cert).unwrap(),
            &std::fs::read(key).unwrap(),
        )
        .unwrap();
        let connector = TlsConnector::builder()
            .identity(identity)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let connector = Connector::NativeTls(connector);

        // Connect to the WebSocket server
        let url = std::format!(
            "wss://{}:443/v1/outlet?node_id={}&node_hostname=ghidora&node_version={}",
            self.cp_url,
            self.node_id,
            self.node_version
        );
        let (ws_stream, _) =
            connect_async_tls_with_config(url, None, false, Option::from(connector))
                .await
                .unwrap();

        println!("Connected to server");
        let (mut write, mut read) = ws_stream.split();

        write
            .send(Message::Binary(
                self.node_info.serialize().unwrap().into_bytes(),
            ))
            .await
            .unwrap();
        let _ = write.flush();
        println!("Sent node info");
        let write = Arc::new(Mutex::new(write));
        let write_clone = Arc::clone(&write);

        let hash = Arc::new(Mutex::new("0".repeat(32)));
        let hash_clone = Arc::clone(&hash);

        tokio::spawn(async move {
            println!("Pinger started");
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                let local_hash = hash_clone.lock().await;
                let value = local_hash.clone();
                let mut write = write_clone.lock().await;
                write.send(Message::Ping(value.into_bytes())).await.unwrap();
                println!("Sent ping");
            }
        });

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

                    // Parse the kong config
                    //TODO: send the config through a channel to a thread thwt will configure Pingora
                    let conf: config::KongConfig =
                        serde_json::from_str(&s).expect("Error decoding kong config");
                    let mut local_hash = hash.lock().await;
                    if *local_hash == "0".repeat(32) {
                        write
                            .lock()
                            .await
                            .send(Message::Ping(conf.config_hash.clone().into()))
                            .await
                            .unwrap();
                        println!("Sent ping to ack the config");
                    }
                    *local_hash = conf.config_hash.clone();
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
    }
}
