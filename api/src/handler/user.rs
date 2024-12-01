use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use garde::Validate;

use kernel::model::id::UserId;
use kernel::model::user::event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::extractor::AuthorizedUser;
use crate::model::user::{
    CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
    UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
};

pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

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
