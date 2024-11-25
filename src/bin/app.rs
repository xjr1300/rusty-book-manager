use std::net::{Ipv4Addr, SocketAddr};

use axum::Router;
use tokio::net::TcpListener;

use adapter::database::connect_database_with;
use api::route::book::build_book_routers;
use api::route::health::build_health_check_routers;
use registry::AppRegistry;
use shared::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await
}

async fn bootstrap() -> anyhow::Result<()> {
    // アプリ設定を構築
    let app_config = AppConfig::new()?;
    // データベースに接続
    let pool = connect_database_with(&app_config.database);
    // AppRegistry(DIコンテナ)を構築
    let registry = AppRegistry::new(pool);

    // ルーターを登録
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .with_state(registry);

    // サーバーを起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .map_err(anyhow::Error::from)
}
