use axum::{routing, Router};

use registry::AppRegistry;

use crate::handler::book::{delete_book, register_book, show_book, show_book_list, update_book};
use crate::handler::checkout::{
    checkout_book, checkout_history, return_book, show_checked_out_list,
};

pub fn build_book_routers() -> Router<AppRegistry> {
    let book_routers = Router::new()
        .route("/", routing::post(register_book))
        .route("/", routing::get(show_book_list))
        .route("/:book_id", routing::get(show_book))
        .route("/:book_id", routing::put(update_book))
        .route("/:book_id", routing::delete(delete_book));
    let checkout_routers = Router::new()
        .route("/checkouts", routing::get(show_checked_out_list))
        .route("/:book_id/checkouts", routing::post(checkout_book))
        .route(
            "/:book_id/checkouts/:checkout_id/returned",
            routing::put(return_book),
        )
        .route("/:book_id/checkout-history", routing::get(checkout_history));
    Router::new().nest("/books", book_routers.merge(checkout_routers))
}
