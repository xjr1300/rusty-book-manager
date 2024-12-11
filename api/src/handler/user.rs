use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use garde::Validate;

use kernel::model::id::UserId;
use kernel::model::user::event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::extractor::AuthorizedUser;
use crate::model::checkout::CheckoutsResponse;
use crate::model::user::{
    CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
    UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
};

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/users/me",
        responses(
            (status = 200, description = "ログインしているユーザーの情報を取得できた場合。", body = UserResponse),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "get current user",
    skip(user),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/users",
        responses(
            (status = 200, description = "ユーザーの一覧の取得に成功した場合。", body = UsersResponse),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "list users",
    skip(_user, registry),
    fields(
        user_id = %_user.user.id.to_string(),
    )
)]
pub async fn list_users(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    let users = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();

    Ok(Json(UsersResponse { users }))
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/v1/users",
        request_body = CreateUserRequest,
        responses(
            (status = 201, description = "ユーザーの登録に成功した場合。", body = UserResponse),
            (status = 400, description = "リクエストしたユーザーに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 403, description = "管理者以外がアクセスした場合。"),
            (status = 422, description = "リクエストしたユーザーの記録に失敗した場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "register user",
    skip(user, registry, body),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(body): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // ユーザーが管理者の場合のみ許可
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    body.validate(&())?;

    let registered_user = registry
        .user_repository()
        .create(CreateUser::from(body))
        .await?;

    Ok(Json(UserResponse::from(registered_user)))
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/users/me/password",
        request_body = UpdateUserPasswordRequest,
        responses(
            (status = 200, description = "ユーザーのパスワードの変更に成功した場合。"),
            (status = 400, description = "リクエストボディに不備があった場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 422, description = "ユーザーのパスワードの記録に失敗した場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "change user password",
    skip(user, registry, body),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn change_password(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(body): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    body.validate(&())?;

    let request = UpdateUserPasswordRequestWithUserId::new(user.id(), body);

    registry
        .user_repository()
        .update_password(UpdateUserPassword::from(request))
        .await?;
    Ok(StatusCode::OK)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/users/{user_id}/role",
        params(
            ("user_id" = Uuid, Path, description = "ロールを変更するユーザーのユーザーID。"),
        ),
        request_body = UpdateUserRoleRequest,
        responses(
            (status = 200, description = "ユーザーのロールの変更に成功した場合。"),
            (status = 400, description = "パスで指定されたユーザーIDまたはリクエストボディの内容に不備がある場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 403, description = "管理者以外がアクセスした場合。"),
            (status = 404, description = "パスで指定されたユーザーIDを持つユーザーが存在しない場合。"),
            (status = 422, description = "ユーザーのロールの記録に失敗した場合。"),
        )

    )
)]
pub async fn change_role(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(body): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    // ユーザーが管理者の場合のみ許可
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    let request = UpdateUserRoleRequestWithUserId::new(user.id(), body);

    registry
        .user_repository()
        .update_role(UpdateUserRole::from(request))
        .await?;

    Ok(StatusCode::OK)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        delete,
        path = "/api/v1/users/{user_id}",
        params(
            ("user_id" = Uuid, Path, description = "削除するユーザーのユーザーID。"),
        ),
        responses(
            (status = 204, description = "ユーザーの削除に成功した場合。"),
            (status = 400, description = "パスで指定されたユーザーIDに不備がある場合。"),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
            (status = 403, description = "管理者以外がアクセスした場合。"),
            (status = 404, description = "パスで指定されたユーザーIDを持つユーザーが存在しない場合。"),
            (status = 422, description = "ユーザーの削除に失敗した場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "delete user",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn delete_user(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    // ユーザーが管理者の場合のみ場一項可能
    if !user.is_admin() {
        return Err(AppError::ForbiddenOperation);
    }

    registry
        .user_repository()
        .delete_user(DeleteUser { user_id })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/users/me/checkouts",
        responses(
            (status = 200, description = "ユーザーが借りている蔵書の一覧の取得に成功した場合。", body = CheckoutsResponse),
            (status = 401, description = "認証されていないユーザーがアクセスした場合。"),
        )
    )
)]
#[tracing::instrument(
    name = "get checked-out books by users",
    skip(user, registry),
    fields(
        user_id = %user.user.id.to_string(),
    )
)]
pub async fn get_checkouts(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_by_user_id(user.id())
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
