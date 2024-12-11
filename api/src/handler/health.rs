use axum::extract::State;
use axum::http::StatusCode;

use registry::AppRegistry;

/// ヘルスチェックハンドラ
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/health",
        responses(
            (status = 200, description = "ヘルスチェックに成功した場合。")
        )
    )
)]
#[tracing::instrument(name = "health check")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// データベースヘルスチェックハンドラ
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/health/db",
        responses(
            (status = 200, description = "データベースのヘルスチェックに成功した場合。"),
        )
    ),

)]
#[tracing::instrument(name = "database health check", skip(registry))]
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    match registry.health_check_repository().check_db().await {
        true => StatusCode::OK,
        false => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
