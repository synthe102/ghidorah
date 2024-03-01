use native_tls;
use std::net::TcpStream;
use tungstenite::{client_tls_with_config, Connector, Message};

fn main() {
    env_logger::init();

    let cert_bytes =
        include_bytes!("../../../kong/koko-wd/internal/certs/clusters/mtls/foo/cluster.crt");
    let key_bytes =
        include_bytes!("../../../kong/koko-wd/internal/certs/clusters/mtls/foo/cluster.key");
    let identity =
        native_tls::Identity::from_pkcs8(cert_bytes, key_bytes).expect("Error creating identity");
    let connector = native_tls::TlsConnector::builder()
        .identity(identity)
        .build()
        .unwrap();

    let stream = TcpStream::connect("foo-pinned.koko.localhost:3100").unwrap();
    let connector: Connector = Connector::NativeTls(connector);

    let (mut  socket, response)= client_tls_with_config("wss://foo-pinned.koko.localhost:3100/v1/outlet?node_hostname=localhost&node_id=00000000-0000-4000-8000-000000000001&node_version=3.6.0.0", stream, None, Some(connector)).unwrap();

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket
        .send(Message::Text("Hello WebSocket".into()))
        .unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
    // socket.close(None);
}

