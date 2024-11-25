use axum::{routing, Router};

use registry::AppRegistry;

use crate::handler::book::{register_book, show_book, show_book_list};

pub fn build_book_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", routing::post(register_book))
        .route("/", routing::get(show_book_list))
        .route("/:book_id", routing::get(show_book));

    Router::new().nest("/books", routers)
}
