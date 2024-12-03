use async_trait::async_trait;
use derive_new::new;

use kernel::model::checkout::event::{CreateCheckout, UpdateReturned};
use kernel::model::checkout::Checkout;
use kernel::model::id::{BookId, CheckoutId, UserId};
use kernel::repository::checkout::CheckoutRepository;
use shared::error::{AppError, AppResult};

use crate::database::checkout::{CheckoutRow, CheckoutStateRow, ReturnedCheckoutRow};
use crate::database::ConnectionPool;

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}

impl CheckoutRepositoryImpl {
    async fn set_transaction_serializable(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
                SET TRANSACTION ISOLATION LEVEL SERIALIZABLE
            "#
        )
        .execute(&mut **tx)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn find_returned_by_book_id(&self, book_id: BookId) -> AppResult<Option<Checkout>> {
        let result = sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM checkouts c
                INNER JOIN books b ON c.book_id = c.book_id
                WHERE c.book_id = $1
            "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .map(Checkout::from);

        Ok(result)
    }
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        // トランザクション分離レベルをシリアライザブルに設定
        self.set_transaction_serializable(&mut tx).await?;

        // 貸出する前に次のブロックで以下を確認する。
        // * 指定の蔵書IDを持つ蔵書が存在するか確認する (リピータブルリードを保証しなくてはならない)。
        // * 存在した場合、その蔵書が貸出中でないか確認する (リピータブルリードを保証しなくてはならない)。
        // 上記をすべて満たす場合、蔵書を貸出する (ファントムリードが発生してはならない)。
        // 他のトランザクションが上記の状態を変更しないように、トランザクション分離レベルをシリアライザブル
        // にする必要がある。
        {
            let result = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id "checkout_id?: CheckoutId",
                        NULL "user_id?: UserId"
                    FROM books b
                    LEFT OUTER JOIN checkouts c ON b.book_id = c.book_id
                    WHERE b.book_id = $1
                "#,
                event.book_id as _
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::SpecificOperationError)?;
            match result {
                None => {
                    return Err(AppError::EntityNotFound(format!(
                        "The book ({}) doesn't exist",
                        event.book_id
                    )))
                }
                Some(CheckoutStateRow {
                    checkout_id: Some(_),
                    ..
                }) => {
                    return Err(AppError::UnprocessableEntity(format!(
                        "The book ({}) was already borrowed",
                        event.book_id
                    )))
                }
                _ => {}
            }
        }

        // 蔵書を貸出
        let checkout_id = CheckoutId::new();
        let result = sqlx::query!(
            r#"
                INSERT INTO checkouts (
                    checkout_id, book_id, user_id, checked_out_at
                ) VALUES (
                    $1, $2, $3, $4
                )
            "#,
            checkout_id as _,
            event.book_id as _,
            event.checked_out_by as _,
            event.checked_out_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if result.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "no checkout record has been created".into(),
            ));
        }

        // トランザクションをコミット
        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        let mut tx = self.db.begin().await?;

        self.set_transaction_serializable(&mut tx).await?;

        // 返却する前に次のブロックで以下を確認する。
        // * 指定の蔵書IDを持つ蔵書が存在するか
        // * 存在中の場合、その蔵書は貸出中であり、借りたユーザーが返却したユーザーと同じか
        {
            let result = sqlx::query_as!(
                CheckoutStateRow,
                r#"
                    SELECT
                        b.book_id,
                        c.checkout_id "checkout_id?: CheckoutId",
                        c.user_id "user_id?: UserId"
                    FROM books b
                    LEFT OUTER JOIN checkouts c ON b.book_id = c.book_id
                    WHERE b.book_id = $1
                "#,
                event.book_id as _
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::SpecificOperationError)?;

            match result {
                None => {
                    return Err(AppError::EntityNotFound(format!(
                        "the book ({}) doesn't exist",
                        event.book_id
                    )))
                }
                Some(CheckoutStateRow {
                    checkout_id: Some(c),
                    user_id: Some(u),
                    ..
                }) if (c, u) != (event.checkout_id, event.returned_by) => {
                    return Err(AppError::UnprocessableEntity(format!(
                        "the user ({}) can not return the book ({}) of the checkout ({})",
                        event.returned_by, event.book_id, event.checkout_id
                    )));
                }
                _ => {}
            }
        }

        // checkoutsテーブルにある当該貸出IDのレコードにreturned_atを設定して、returned_checkouts
        // テーブルに行を登録
        let result = sqlx::query!(
            r#"
                INSERT INTO returned_checkouts (
                    checkout_id, book_id, user_id, checked_out_at, returned_at
                )
                SELECT
                    checkout_id, book_id, user_id, checked_out_at, $2
                FROM checkouts
                WHERE checkout_id = $1
            "#,
            event.checkout_id as _,
            event.returned_at
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if result.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "not returning record has been updated".into(),
            ));
        }

        // checkoutsテーブルから当該貸出IDのレコードを削除
        let result = sqlx::query!(
            r#"
                DELETE FROM checkouts
                WHERE checkout_id = $1
            "#,
            event.checkout_id as _
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if result.rows_affected() < 1 {
            return Err(AppError::NoRowsAffectedError(
                "no checkout record has been deleted".into(),
            ));
        }

        tx.commit().await.map_err(AppError::TransactionError)?;

        Ok(())
    }

    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>> {
        sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM checkouts c
                INNER JOIN books b ON c.book_id = b.book_id
                ORDER BY c.checked_out_at
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map(|rows| rows.into_iter().map(Checkout::from).collect())
        .map_err(AppError::SpecificOperationError)
    }

    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {
        sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT
                    c.checkout_id,
                    c.book_id,
                    c.user_id,
                    c.checked_out_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM checkouts c
                INNER JOIN books b ON c.book_id = b.book_id
                WHERE c.user_id = $1
                ORDER BY c.checked_out_at
            "#,
            user_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map(|rows| rows.into_iter().map(Checkout::from).collect())
        .map_err(AppError::SpecificOperationError)
    }

    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>> {
        // 蔵書について未返却の貸出を取得
        let checkout = self.find_returned_by_book_id(book_id).await?;

        // 返却済みの貸出を取得
        let mut checkout_histories: Vec<Checkout> = sqlx::query_as!(
            ReturnedCheckoutRow,
            r#"
                SELECT
                    rc.checkout_id,
                    rc.book_id,
                    rc.user_id,
                    rc.checked_out_at,
                    rc.returned_at,
                    b.title,
                    b.author,
                    b.isbn
                FROM returned_checkouts rc
                INNER JOIN books b ON rc.book_id = b.book_id
                WHERE rc.book_id = $1
                ORDER BY rc.checked_out_at DESC
            "#,
            book_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .into_iter()
        .map(Checkout::from)
        .collect();

        if let Some(checkout) = checkout {
            checkout_histories.insert(0, checkout);
        }

        Ok(checkout_histories)
    }
}
