use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use kernel::model::id::BookId;
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::model::book::{BookResponse, CreateBookRequest};

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> AppResult<Json<Vec<BookResponse>>> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| {
            v.into_iter()
                .map(BookResponse::from)
                .collect::<Vec<BookResponse>>()
        })
        .map(Json)
}

pub async fn show_book(
    State(registry): State<AppRegistry>,
    Path(book_id): Path<BookId>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found".into(),
            )),
        })
}
