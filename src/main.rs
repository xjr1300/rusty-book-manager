use std::net::{Ipv4Addr, SocketAddr};

use axum::extract::State;
use axum::http::StatusCode;
use axum::{routing, Router};
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;

/// データベース接続設定
struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}

/// ヘルスチェックハンドラ
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// データベースヘルスチェックハンドラ
async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // データベース接続設定を定義
    let database_cfg = DatabaseConfig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };
    // データベースコネクションプールを構築
    let conn_pool = connect_database_with(database_cfg);

    let app = Router::new()
        .route("/health", routing::get(health_check))
        .route("/health/db", routing::get(health_check_db))
        .with_state(conn_pool);
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
