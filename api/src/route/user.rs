use axum::routing;
use axum::Router;

use registry::AppRegistry;

use crate::handler::user::{
    change_password, change_role, delete_user, get_current_user, list_users, register_user,
};

pub fn build_user_routers() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", routing::get(get_current_user))
        .route("/users/me/password", routing::put(change_password))
        .route("/users", routing::get(list_users).post(register_user))
        .route("/users/:user_id", routing::delete(delete_user))
        .route("/users/:user_id/role", routing::put(change_role))
}
