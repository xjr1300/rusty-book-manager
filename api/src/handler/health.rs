use axum::extract::State;
use axum::http::StatusCode;

use registry::AppRegistry;

/// ヘルスチェックハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// データベースヘルスチェックハンドラ
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    match registry.health_check_repository().check_db().await {
        true => StatusCode::OK,
        false => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
