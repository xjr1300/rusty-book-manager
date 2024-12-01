use axum::Router;

use registry::AppRegistry;

use super::book::build_book_routers;
use super::health::build_health_check_routers;
use super::user::build_user_routers;

pub fn routers() -> Router<AppRegistry> {
    let router = Router::new()
        .merge(build_health_check_routers())
        .merge(build_user_routers())
        .merge(build_book_routers());
    Router::new().nest("/api/v1", router)
}
