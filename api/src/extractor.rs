use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use kernel::model::auth::AccessToken;
use kernel::model::id::UserId;
use kernel::model::role::Role;
use kernel::model::user::User;
use registry::AppRegistry;
use shared::error::AppError;

pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }

    pub fn is_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AppError;

    /// ハンドラーメソッドの引数に`AuthorizedUser`を追加したときに呼び出されるメソッド
    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        // HTTPヘッダーからアクセストークンを取得
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthorizedError)?;
        let access_token = AccessToken(bearer.token().to_string());

        // アクセストークンに紐づくユーザーIDを取得
        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        // ユーザーIDを基にデータベースからユーザーを取得
        // データベースにユーザーが記録されていない場合は認可されていないことを示すエラーを返す
        let user = registry
            .user_repository()
            .find_current_user(user_id)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;
        Ok(AuthorizedUser { access_token, user })
    }
}
