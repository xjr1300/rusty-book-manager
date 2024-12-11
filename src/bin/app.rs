use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use anyhow::Context;
use axum::http::Method;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_redoc::{Redoc, Servable};

use adapter::database::connect_database_with;
use adapter::redis::RedisClient;
#[cfg(debug_assertions)]
use api::openapi::ApiDoc;
use api::route::{auth, v1};
use registry::AppRegistryImpl;
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

    let jaeger_host = std::env::var("JAEGER_HOST")?;
    let jaeger_port = std::env::var("JAEGER_PORT")?;
    let jaeger_endpoint = format!("{jaeger_host}:{jaeger_port}");

    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(jaeger_endpoint)
        .with_service_name("rusty-book-manager")
        .with_auto_split_batch(true)
        .with_max_packet_size(8192)
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // 環境変数RUST_LOGからロガーのログレベルを設定
    // 環境変数RUST_LOGが設定されていない場合、動作環境から得られたデフォルトのログレベルを設定
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログの出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    // 開発環境では非構造化ログ、プロダクション環境では構造化ログを出力する条件付きコンパイル属性
    // 開発環境でも構造化ログを出力するように無効化
    // #[cfg(not(debug_assertions))]
    let subscriber = subscriber.json();

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(opentelemetry)
        .try_init()?;

    Ok(())
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

async fn bootstrap() -> anyhow::Result<()> {
    // アプリ設定を構築
    let app_config = AppConfig::new()?;
    // データベースに接続
    let pool = connect_database_with(&app_config.database);
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    // AppRegistry(DIコンテナ)を構築
    let registry = Arc::new(AppRegistryImpl::new(pool, kv, app_config));

    let router = Router::new().merge(v1::routers()).merge(auth::routes());
    #[cfg(debug_assertions)]
    let router = router.merge(Redoc::with_url("/docs", ApiDoc::openapi()));

    // ルーターを登録
    let app = router
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
        .layer(cors())
        .with_state(registry);

    // サーバーを起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Unexpected error happened in server")
        .inspect_err(
            |e| tracing::error!(error.cause_chain = ?e, error_message = %e, "Unexpected error"),
        )
}

async fn shutdown_signal() {
    fn purge_spans() {
        opentelemetry::global::shutdown_tracer_provider();
    }

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await
            .expect("Failed to receive SIGTERM signal");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("CTRL+C was received");
            purge_spans()
        }
        _ = terminate => {
            tracing::info!("SIGTERM was received");
            purge_spans()
        }
    }
}
