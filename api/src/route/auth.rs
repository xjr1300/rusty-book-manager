use axum::{routing, Router};

use registry::AppRegistry;

use crate::handler::auth::{login, logout};

pub fn build_auth_routes() -> Router<AppRegistry> {
    let auth_router = Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout));
    Router::new().nest("/auth", auth_router)
}
