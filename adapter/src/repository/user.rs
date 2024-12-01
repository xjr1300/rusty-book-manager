use async_trait::async_trait;
use derive_new::new;

use kernel::model::id::UserId;
use kernel::model::role::Role;
use kernel::model::user::event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole};
use kernel::model::user::User;
use kernel::repository::user::UserRepository;
use shared::error::{AppError, AppResult};

use crate::database::model::user::UserRow;
use crate::database::ConnectionPool;

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    u.user_id,
                    u.name,
                    u.email,
                    r.name as role_name,
                    u.created_at,
                    u.updated_at
                FROM
                    users u
                INNER JOIN roles r ON u.role_id = r.role_id
                WHERE u.user_id = $1
            "#,
            current_user_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(User::try_from(r)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    u.user_id,
                    u.name,
                    u.email,
                    r.name as role_name,
                    u.created_at,
                    u.updated_at
                FROM
                    users u
                INNER JOIN roles r on u.role_id = r.role_id
                ORDER BY u.created_at
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .into_iter()
        .filter_map(|r| User::try_from(r).ok())
        .collect::<Vec<User>>();

        Ok(users)
    }

    async fn create(&self, event: CreateUser) -> AppResult<User> {
        let user_id = UserId::new();
        let hashed_password = hash_password(&event.password)?;
        let role = Role::User;

        let result = sqlx::query!(
            r#"
                INSERT INTO users (user_id, name, email, password_hash, role_id)
                SELECT $1, $2, $3, $4, role_id FROM roles WHERE name = $5
            "#,
            user_id as _,
            event.name,
            event.email,
            hashed_password,
            role.as_ref()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if result.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "no user has been created".into(),
            ));
        }

        Ok(User {
            id: user_id,
            name: event.name,
            email: event.email,
            role,
        })
    }

    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        // ユーザーの現在のハッシュ化されたパスワードをデータベースから取得
        let original_password_hash = sqlx::query!(
            r#"
                SELECT
                    password_hash
                FROM
                    users
                WHERE
                    user_id = $1
            "#,
            event.user_id as _
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?
        .password_hash;

        // 入力されたパスワードが等しいか確認
        verify_password(&event.current_password, &original_password_hash)?;

        // 新しいパスワードをハッシュ化して、データベースに保存
        let new_password_hash = hash_password(&event.new_password)?;
        sqlx::query!(
            r#"
                UPDATE users
                SET password_hash = $1
                WHERE user_id = $2
            "#,
            new_password_hash,
            event.user_id as _
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    async fn update_role(&self, event: UpdateUserRole) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
                UPDATE users
                SET role_id = (
                        SELECT role_id
                        FROM roles
                        WHERE name = $2
                    )
                WHERE user_id = $1
            "#,
            event.user_id as _,
            event.role.as_ref()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        if result.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified user not found".into()));
        }

        Ok(())
    }

    async fn delete_user(&self, event: DeleteUser) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM users WHERE user_id = $1
            "#,
            event.user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        if result.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified user not found".into()));
        }

        Ok(())
    }
}

fn hash_password(password: &str) -> AppResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(AppError::from)
}

fn verify_password(password: &str, hash: &str) -> AppResult<()> {
    let valid = bcrypt::verify(password, hash)?;
    if !valid {
        return Err(AppError::UnauthenticatedError);
    }
    Ok(())
}
