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

use clap::Parser;
use once_cell::sync::Lazy;
use pingora::prelude::*;
use tokio::runtime::{Builder, Runtime};

pub mod config;
pub mod gateway;

// Initialize the tokio runtime
static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .thread_name("tokio")
        .enable_all()
        .build()
        .unwrap()
});

fn main() {
    let args = config::Args::parse();
    let node_info = gateway::node::NodeInfo::default();
    let gateway = gateway::Konglet::new(
        node_info,
        String::from("3.6.0.0"),
        args.control_plane_url,
        args.cert,
        args.key,
    );
    TOKIO_RUNTIME.spawn(async move {
        gateway.run().await;
    });
    // Start the Pingora server
    let mut server = Server::new(None).unwrap();
    server.bootstrap();
    // Add pinger as a backgroup service
    // let background_service = background_service("Pinger", gw.pinger);
    // server.add_service(background_service);
    println!("Pingora server starting");
    server.run_forever();

    //TODO: hot reload the Pingora server when the kong config changes
}
