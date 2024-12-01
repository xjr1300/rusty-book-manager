use axum::{routing, Router};

use registry::AppRegistry;

use crate::handler::book::{delete_book, register_book, show_book, show_book_list, update_book};

pub fn build_book_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", routing::post(register_book))
        .route("/", routing::get(show_book_list))
        .route("/:book_id", routing::get(show_book))
        .route("/:book_id", routing::put(update_book))
        .route("/:book_id", routing::delete(delete_book));
    Router::new().nest("/books", routers)
}
