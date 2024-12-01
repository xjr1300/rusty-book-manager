use std::sync::Arc;

use async_trait::async_trait;
use derive_new::new;

use kernel::model::auth::event::CreateToken;
use kernel::model::auth::AccessToken;
use kernel::model::id::UserId;
use kernel::repository::auth::AuthRepository;
use shared::error::{AppError, AppResult};

use crate::database::model::auth::{from, AuthorizationKey, AuthorizedUserId, UserItem};
use crate::database::ConnectionPool;
use crate::redis::RedisClient;

#[derive(new)]
pub struct AuthRepositoryImpl {
    db: ConnectionPool,
    kv: Arc<RedisClient>,
    ttl: u64,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    /// `create_token`で保存したアクセストークンがRedis内にキーとして存在すれば、その値である`UserId`を返す。
    /// Redisへの保存時に設定したアクセストークンの有効期限が切れていたり、アクセストークンが誤っている場合は、
    /// `Option::None`を返す。
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AppResult<Option<UserId>> {
        let key: AuthorizationKey = access_token.into();
        self.kv
            .get(&key)
            .await
            .map(|x| x.map(AuthorizedUserId::into_inner))
    }

    /// メールアドレスとパスワードから、該当するユーザーが存在することを確認する。
    /// パスワードはbcryptでハッシュ化されてデータベースに記録されているため、ハッシュ化前のパスワードと
    /// 一致するか確認する。
    async fn verify_user(&self, email: &str, password: &str) -> AppResult<UserId> {
        let user_item = sqlx::query_as!(
            UserItem,
            r#"
                SELECT user_id, password_hash
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let valid = bcrypt::verify(password, &user_item.password_hash)?;
        if !valid {
            return Err(AppError::UnauthorizedError);
        }
        Ok(user_item.user_id)
    }

    /// アクセストークンを生成してRedisに保存して、アクセストークンを返す。
    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken> {
        let (key, value) = from(event);
        self.kv.set_ex(&key, &value, self.ttl).await?;
        Ok(key.into())
    }

    /// Redisに登録されているアクセストークンを削除する。
    async fn delete_token(&self, access_token: AccessToken) -> AppResult<()> {
        let key: AuthorizationKey = access_token.into();
        self.kv.delete(&key).await
    }
}
