use std::net::{Ipv4Addr, SocketAddr};

use axum::http::StatusCode;
use axum::{routing, Router};
use tokio::net::TcpListener;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/health", routing::get(health_check));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_works() {
        let status_code = health_check().await;
        assert_eq!(status_code, StatusCode::OK);
    }
}
