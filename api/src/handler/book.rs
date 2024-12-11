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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path="/api/v1/books",
        params(
            ("limit" = i64, Query, description = "一度に取得する蔵書数の上限値の指定"),
            ("offset" = i64, Query, description = "取得対象とする蔵書一覧の開始位置"),
        ),
        responses(
            (status = 200, description = "蔵書一覧の取得に成功した場合。", body = PaginatedBookResponse),
            (status = 400, description = "クエリに指定された上限値または開始位置に不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        ),
    )
)]
#[tracing::instrument(
    name = "show book list",
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string()
    )
)]
pub async fn show_book_list(
    _user: AuthorizedUser,
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books/{book_id}",
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID"),
        ),
        responses(
            (status = 200, description = "指定された蔵書の取得に成功した場合。", body = BookResponse),
            (status = 400, description = "パスで指定された蔵書IDに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 404, description = "パスで指定された蔵書IDを持つ蔵書が存在しない場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "show book",
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string()
    )
)]
pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    tracing::info!("ここにINFOログを追加しました。");
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

/// The HyperText Transfer Protocol (HTTP) の 422 Unprocessable Entity 応答状態コードは、
/// サーバーが要求本文のコンテンツ型を理解でき、要求本文の構文が正しいものの、中に含まれている指示が
/// 処理できなかったことを表します。
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/v1/books",
        request_body = CreateBookRequest,
        responses(
            (status = 201, description = "蔵書の登録に成功した場合。"),
            (status = 400, description = "リクエストした蔵書に不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 422, description = "リクエストした蔵書の記録に失敗した場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "register book",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string()
    )
)]
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/books/{book_id}",
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID"),
        ),
        request_body = UpdateBookRequest,
        responses(
            (status = 200, description = "蔵書の更新に成功した場合。"),
            (status = 400, description = "パスで指定された蔵書IDまたはリクエストボディに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 404, description = "パスで指定された蔵書IDを持つ蔵書が存在しない場合。"),
            (status = 422, description = "蔵書の記録に失敗した場合。"),

        )
    )
)]
#[tracing::instrument(
    name = "update book",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
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

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        delete,
        path = "/api/v1/books/{book_id}",
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID"),
        ),
        responses(
            (status = 204, description = "蔵書の削除に成功した場合。"),
            (status = 400, description = "パスで指定した蔵書IDに不備があった場合。"),
            (status = 404, description = "パスで指定した蔵書IDを持つ蔵書が存在しない場合。"),
            (status = 422, description = "蔵書を削除できなかった場合。"),
        ),
    )
)]
#[tracing::instrument(
    name = "delete book",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string()
    )
)]
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
