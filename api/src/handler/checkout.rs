use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;

use kernel::model::checkout::event::{CreateCheckout, UpdateReturned};
use kernel::model::id::{BookId, CheckoutId};
use registry::AppRegistry;
use shared::error::AppResult;

use crate::extractor::AuthorizedUser;
use crate::model::checkout::CheckoutsResponse;

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/v1/books/{book_id}/checkouts",
        params(
            ("book_id" = Uuid, Path, description = "貸出する蔵書ID"),
        ),
        responses(
            (status = 201, description = "蔵書の貸出に成功した場合。"),
            (status = 400, description = "パスで指定された蔵書IDに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 404, description = "パスで指定された蔵書IDを持つ蔵書が存在しない場合。"),
            (status = 422, description = "蔵書の貸出を記録できなかった場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "checkout book",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn checkout_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let event = CreateCheckout::new(book_id, user.id(), Utc::now());
    registry
        .checkout_repository()
        .create(event)
        .await
        .map(|_| StatusCode::CREATED)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/books/{book_id}/checkouts/{checkout_id}/returned",
        params(
            ("book_id" = Uuid, Path, description = "貸し出した蔵書ID"),
            ("checkout_id" = Uuid, Path, description = "貸出ID"),
        ),
        responses(
            (status = 200, description = "貸出した蔵書の返却に成功した場合。"),
            (status = 400, description = "パスで指定された蔵書IDまたは貸出IDに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 404, description = "パスで指定された蔵書IDを持つ蔵書、または貸出IDを持つ貸出が存在しなかった場合。"),
            (status = 422, description = "貸出した蔵書の返却の記録に失敗した場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "return book",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn return_book(
    user: AuthorizedUser,
    Path((book_id, checkout_id)): Path<(BookId, CheckoutId)>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let event = UpdateReturned::new(checkout_id, book_id, user.id(), Utc::now());
    registry
        .checkout_repository()
        .update_returned(event)
        .await
        .map(|_| StatusCode::OK)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books/checkouts",
        responses(
            (status = 200, description = "貸し出した蔵書の一覧の取得に成功した場合。", body = CheckoutsResponse),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        ),
    )
)]
#[tracing::instrument(
    name = "show checked-out list",
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string(),
    )
)]
pub async fn show_checked_out_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_all()
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books/{book_id}/checkout-history",
        params(
            ("book_id" = Uuid, Path, description = "蔵書ID"),
        ),
        responses(
            (status = 200, description = "指定した蔵書の貸出履歴。", body = CheckoutsResponse),
            (status = 400, description = "パスで指定した蔵書IDに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 404, description = "パスで指定した蔵書IDを持つ蔵書が存在しない場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "checkout history",
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string(),
    )
)]
pub async fn checkout_history(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_history_by_book_id(book_id)
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
