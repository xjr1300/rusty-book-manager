#[cfg(test)]
mod tests {
    #[sqlx::test]
    async fn sqlx_test_works(pool: sqlx::PgPool) {
        let row = sqlx::query!("SELECT 1 + 1 result")
            .fetch_one(&pool)
            .await
            .unwrap();
        let result = row.result;
        assert_eq!(result, Some(2));
    }

    #[sqlx::test(fixtures("common"))]
    async fn retrieve_book_rows(pool: sqlx::PgPool) {
        let row = sqlx::query!(
            r#"
                SELECT
                    author
                FROM
                    books
                WHERE
                    title = 'Test Book 1'
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        let result = row.author;
        assert_eq!(result, "Test Author 1".to_string());
    }
}
