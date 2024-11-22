use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing, Router};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", routing::get(hello_world));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
