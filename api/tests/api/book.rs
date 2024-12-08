/*
use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;
use rstest::rstest;
use tower::ServiceExt;

use api::model::book::PaginatedBookResponse;
use kernel::model::book::Book;
use kernel::model::id::{BookId, UserId};
use kernel::model::list::PaginatedList;
use kernel::model::user::BookOwner;
use kernel::repository::book::MockBookRepository;

use crate::deserialize_json;
use crate::helper::{fixture, make_router, v1, TestRequestExt};

#[rstest]
#[case("/books", 20, 0)]
#[case("/books?limit=50", 50, 0)]
#[case("/books?limit=50&offset=20", 50, 20)]
#[case("/books?offset=20", 20, 20)]
#[tokio::test]
async fn show_book_list_with_query_200(
    // 1. fixtureとしてmockオブジェクトを渡している
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
    #[case] expected_limit: i64,
    #[case] expected_offset: i64,
) -> anyhow::Result<()> {
    let book_id = BookId::new();

    // 2. モックの挙動を設定する
    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_find_all().returning(move |opt| {
            let items = vec![Book {
                id: book_id,
                title: "RustによるWebアプリケーション開発".to_string(),
                isbn: "".to_string(),
                author: "Yuki Toyoda".to_string(),
                description: "RustによるWebアプリケーション開発".to_string(),
                owner: BookOwner {
                    id: UserId::new(),
                    name: "Yuki Toyoda".to_string(),
                },
                checkout: None,
            }];
            Ok(PaginatedList {
                total: 1,
                limit: opt.limit,
                offset: opt.offset,
                items,
            })
        });
        Arc::new(mock)
    });

    // 3. ルーターを作成する
    let app: axum::Router = make_router(fixture);

    // 4. リクエストを作成・送信し、レスポンスのステータスコードを検証する
    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let resp = app.oneshot(req).await?;
    assert_eq!(resp.status(), axum::http::StatusCode::OK);

    // 5. レスポンスの値を検証する
    let result = deserialize_json!(resp, PaginatedBookResponse);
    assert_eq!(result.limit, expected_limit);
    assert_eq!(result.offset, expected_offset);

    // 6. テストが成功していることを示す
    Ok(())
}
*/
