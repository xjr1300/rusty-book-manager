use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use garde::Validate;

use kernel::model::book::event::{DeleteBook, UpdateBook};
use kernel::model::book::BookListOptions;
use kernel::model::id::BookId;
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::extractor::AuthorizedUser;
use crate::model::book::{
    BookListQuery, BookResponse, CreateBookRequest, PaginatedBookResponse, UpdateBookRequest,
    UpdateBookRequestWithIds,
};

pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(body): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    body.validate(&())?;

    registry
        .book_repository()
        .create(body.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    Query(query): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<PaginatedBookResponse>> {
    query.validate(&())?;

    registry
        .book_repository()
        .find_all(BookListOptions::from(query))
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
}

pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|b| match b {
            Some(b) => Ok(Json(b.into())),
            None => Err(AppError::EntityNotFound(
                "the specified book was not fount".into(),
            )),
        })
}

pub async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(body): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    body.validate(&())?;

    let update_book = UpdateBookRequestWithIds::new(book_id, user.id(), body);

    registry
        .book_repository()
        .update(UpdateBook::from(update_book))
        .await
        .map(|_| StatusCode::OK)
}

pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let delete_book = DeleteBook {
        book_id,
        requested_user: user.id(),
    };

    registry
        .book_repository()
        .delete(delete_book)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}
