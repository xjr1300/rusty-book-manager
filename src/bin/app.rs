use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use adapter::database::connect_database_with;
use api::route::book::build_book_routers;
use api::route::health::build_health_check_routers;
use registry::AppRegistry;
use shared::config::AppConfig;
use shared::env::{which, Environment};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> anyhow::Result<()> {
    // 動作環境からデフォルトのログレベルを定義
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    // 環境変数RUST_LOGからロガーのログレベルを設定
    // 環境変数RUST_LOGが設定されていない場合、動作環境から得られたデフォルトのログレベルを設定
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログの出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registry);

    // サーバーを起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .context("Unexpected error happened n server")
        .inspect_err(
            |e| tracing::error!(error.cause_chain = ?e, error_message = %e, "Unexpected error"),
        )
}
