use async_trait::async_trait;
use derive_new::new;

use kernel::model::book::event::{CreateBook, DeleteBook, UpdateBook};
use kernel::model::book::Book;
use kernel::model::book::BookListOptions;
use kernel::model::id::{BookId, UserId};
use kernel::model::list::PaginatedList;
use kernel::repository::book::BookRepository;
use shared::error::{AppError, AppResult};

use crate::database::model::book::{BookRow, PaginatedBookRow};
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook, user_id: UserId) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (
                    title, author, isbn, description, user_id
                ) VALUES (
                    $1, $2, $3, $4, $5
                )
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            user_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn find_all(&self, options: BookListOptions) -> AppResult<PaginatedList<Book>> {
        let BookListOptions { limit, offset } = options;

        let rows: Vec<PaginatedBookRow> = sqlx::query_as!(
            PaginatedBookRow,
            r#"
                SELECT
                    COUNT(*) OVER() "total!",
                    b.book_id id
                FROM
                    books b
                ORDER BY b.created_at DESC
                LIMIT $1
                OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        // 上記クエリで行が得られた場合は最初の行のtotalカラムの値を、
        // 行が得られなかった場合は`i64`のデフォルト値、つまり0を`total`に割り当てる。
        let total = rows.first().map(|r| r.total).unwrap_or_default();
        let book_ids = rows.into_iter().map(|r| r.id).collect::<Vec<BookId>>();

        // UNNEST: 配列を行集合に展開する。
        // $1::uuid[]: クエリパラメーター$1をuuidの配列と認識させる。
        // 次のSQLは、上記で得られた蔵書IDのベクタをSQL文のパラメーターとして与え、
        // 上記で得られた蔵書IDのベクタに含まれた蔵書IDを持つ蔵書のみを選択する。
        // 蔵書数が膨大にならない限り非効率と考える。次のSQL文を実行して、必要な行を選択するべきではないか。
        // * すべての蔵書の冊数を選択するSQL文
        // * `created_at`の降順でソートした結果に対して、`OFFSET`と`LIMIT`で
        //   ページにレンダリングする蔵書を選択するSQL文
        // SQL文の実行に時間がかかるようになった場合、クエリの実行計画を確認して、SQL文を見直すべきである。
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id, b.title, b.author, b.isbn, b.description,
                    u.user_id owned_by, u.name owner_name
                FROM
                    books b
                INNER JOIN
                    users u ON b.user_id = u.user_id
                WHERE
                    b.book_id IN (SELECT * FROM UNNEST($1::uuid[]))
                ORDER BY b.created_at
            "#,
            &book_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let items = rows.into_iter().map(Book::from).collect();

        Ok(PaginatedList {
            total,
            limit,
            offset,
            items,
        })
    }

    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id, b.title, b.author, b.isbn, b.description,
                    u.user_id owned_by, u.name owner_name
                FROM
                    books b
                INNER JOIN users u ON b.user_id = u.user_id
                WHERE b.book_id = $1
            "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from))
    }

    async fn update(&self, event: UpdateBook) -> AppResult<()> {
        // 蔵書の所有者のみが更新できるように`user_id`を更新条件に含められている。
        let result = sqlx::query!(
            r#"
                UPDATE books
                SET
                    title = $1,
                    author = $2,
                    isbn = $3,
                    description = $4
                WHERE
                    book_id = $5
                    AND user_id = $6
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            event.book_id as _,
            event.requested_user as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if result.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified book not found".into()));
        }

        Ok(())
    }

    async fn delete(&self, event: DeleteBook) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE book_id = $1
            "#,
            event.book_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if result.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified book not found".into()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use kernel::model::user::event::CreateUser;
    use kernel::repository::user::UserRepository;

    use super::*;
    use crate::repository::user::UserRepositoryImpl;

    #[sqlx::test]
    async fn test_register_book(pool: PgPool) -> anyhow::Result<()> {
        // TODO: ロールを追加（フィクスチャーに変更）
        sqlx::query!(
            r#"
                INSERT INTO roles (name)
                VALUES
                    ('Admin'),
                    ('User')
            "#,
        )
        .execute(&pool)
        .await?;
        // TODO: テスト用のユーザーを登録（フィクスチャーに変更）
        let user_repo = UserRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let user = user_repo
            .create(CreateUser {
                name: "Test User".into(),
                email: "test@example.com".into(),
                password: "test_password".into(),
            })
            .await?;
        // TODO: テスト用の蔵書を登録（フィクスチャーに変更）
        let book_repo = BookRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let book = CreateBook {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Description".into(),
        };
        book_repo.create(book, user.id).await?;

        // 蔵書リストを取得
        let options = BookListOptions {
            limit: 20,
            offset: 0,
        };
        let books = book_repo.find_all(options).await?;
        assert_eq!(1, books.items.len());

        let book_id = books.items[0].id;
        let book = book_repo.find_by_id(book_id).await?;
        assert!(book.is_some());

        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner,
        } = book.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");
        assert_eq!(owner.name, "Test User");

        Ok(())
    }
}
