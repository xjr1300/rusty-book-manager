# Rust による Web アプリケーション開発

## リクエスト例

```sh
# アクセストークンのリクエスト
curl --include -H 'Content-Type: application/json' -d '{"email":"eleazar.fig@example.com","password":"password"}' http://localhost:8080/auth/login

# 蔵書の登録リクエスト
# アクセストークン: 6a2c7e0af361414fb787ee8ac56f40a8
curl --include -X POST -H 'Content-Type: application/json' -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' -d '{"title":"RustによるWebアプリケーション開発","author":"豊田優貴／松本健太郎／吉川哲史","isbn":"978-4-06-536957-9","description":"Rustによるアプリケーション開発のベストプラクティス!\\n経験豊富な筆者が貴重な知識とテクニックを伝授。「蔵書管理アプリケーション」の実装を通じて、設計、開発、保守、運用までハンズオンで学ぶ!\\n今こそ現場にRustを!"}' http://localhost:8080/api/v1/books

# 蔵書リストの取得リクエスト
curl --include -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books

# 蔵書を借りるリクエスト
# 蔵書ID: 5490901bdab84bdaa96090b2647d5887
curl --include -X POST -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books/5490901bdab84bdaa96090b2647d5887/checkouts

# 貸出中蔵書リストの取得リクエスト
curl --include -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books/checkouts

# 貸出中の蔵書を返却するリクエスト
# 貸出ID: cb7823cb1eee48a99e34b74392637320
curl --include -X PUT -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books/5490901bdab84bdaa96090b2647d5887/checkouts/cb7823cb1eee48a99e34b74392637320/returned
```

## 要点

### ワークスペースの依存関係

* ワークスペースの`Cargo.toml`にすべての依存関係を追加
* ワークスペースのメンバーであるクレートは、ワークスペースの依存関係を参照

```toml
# workspace.toml
[workspace.dependencies]
anyhow = "1.0.75"
axum = { version = "0.7.5", features = ["macros"] }
thiserror = "1.0.44"

[dependencies]
anyhow.workspace = true
axum.workspace = true
thiserror.workspace = true
```

```toml
# member.toml
[dependencies]
anyhow.workspace = true
axum.workspace = true
thiserror.workspace = true
```

### タスクランナー

* `cargo-make`
* 環境変数の設定
* タスクの依存関係を整理
* プロジェクトルートの`Makefile.toml`に環境変数やタスクを定義

### ミドルウェア

* `tower`クレート
* レイヤを階層上に構築することで、リクエストを受信したとき、レスポンスを返すときにレイヤで処理を挟むことが可能
* HTTPヘッダのBearerトークンやセッションIDからユーザーを特定して、レスポンスハンドラに渡すときに利用
  * `TypedHeader`を使用すると便利

### Pin

* `Pin`型の値がムーブしても、ラップした内部の値を決してムーブしないことを宣言するときに使用する型
* [RustのPinチョットワカル](https://tech-blog.optim.co.jp/entry/2020/03/05/160000)

### 並行プログラミング

* [並行プログラミング入門](https://www.oreilly.co.jp//books/9784873119595/)

### レスポンス型への変換

* [IntoResponseトレイト](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)
* 上記で、`IntoResponse`が実装されている型を確認

### エラー処理

* [anyhow](https://docs.rs/anyhow/latest/anyhow/)クレートの`anyhow::Result<()>`などを`main`関数やテスト関数の戻り値として利用
* [thiserror](https://docs.rs/thiserror/latest/thiserror/)クレートの`thiserror::Error`を利用して、アプリ独自のエラー型を定義
* サードパーティクレートから得られるエラー型を、`#[from]`や`#[source]`を利用して、アプリ独自のエラー型に内包

### テスト

* [cargo-nextest](https://nexte.st/)を使用して、テストを実行（`cargo`が提供する`test`コマンドよりも高機能）
* 非同期処理を伴うテストは`#[tokio::test]`属性を付与
* 実際のデータベースを使用する統合テストの実装は`#[sqlx::test]`属性を付与
  * `#[sqlx::test]`属性を付与すると、テスト関数を非同期化して、データベースコネクションプールをテスト関数の引数を介して提供してくれる
* [rstest](https://docs.rs/rstest/latest/rstest/)クレートを使用して、フィクスチャーを扱い、パラメータ化テストを実装

### DI

* 依存関係の注入は静的ディスパッチにすると型の解決が煩雑化
* 依存関係の注入は動的ディスパッチにすると、実行時に型の解決が行われるため、型の解決が容易になるが、実行時にオーバーヘッドが発生
* リポジトリが定義するメソッドを提供するトレイトは`Send + Sync`を満たす必要がある
* トレイトやトレイトの実装にを非同期化するためには、[async-trait](https://docs.rs/async-trait/latest/async_trait/)クレートを利用

### sqlx

* [sqlx](https://docs.rs/sqlx/latest/sqlx/)クレートを使用して、PostgreSQLデータベースとの接続を行う
* `rust-analyzer`が`sqlx`のマクロを正しく解釈できない場合は、`.cargo/config.toml`ファイルにデータベースへの接続情報を記述

```toml
[env]
DATABASE_URL = "postgres://<username>:<password>@<host>:<port>/<database>"
```

* ドメインプリミティブを定義した場合は、構造体に`sqlx::Type`を導出して、`#[sqlx(transparent)]`属性を付与
* ドメインプリミティブを`sqlx::query!`マクロなどのSQLクエリのバインドパラメータとして使用する場合は、`as _`を使用してコンパイラのチェックを無効化

```rust
#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct BookId(uuid::Uuid);

let rows: Vec<BookRow> = sqlx::query_as!(
    BookRow,
    r#"
        SELECT b.book_id, b.title
        FROM books b
        WHERE b.book_id = $1
    "#,
    &book_ids as _
)
.fetch_all(self.db.inner_ref())
.await?;
```

* `sqlx`がNULL許容として判断したフィールドを非NULLと判断させたい場合は`foo as foo!`とする。
* `sqlx`が非NULLとして判断したフィールドをNULL許容と判断させたい場合は`foo as foo?`とする。

```rust
// Postgres: using a raw query string lets us use unescaped double-quotes
// Note that this query wouldn't work in SQLite as we still don't know the exact type of `id`
let record = sqlx::query!(r#"select 1 as "id!""#) // MySQL: use "select 1 as `id!`" instead
    .fetch_one(&mut conn)
    .await?;

// For Postgres this would have been inferred to be Option<i32> instead
assert_eq!(record.id, 1i32);

// Postgres/SQLite:
let record = sqlx::query!(r#"select 1 as "id?""#) // MySQL: use "select 1 as `id?`" instead
    .fetch_one(&mut conn)
    .await?;

// For Postgres this would have been inferred to be Option<i32> anyway
// but this is just a basic example
assert_eq!(record.id, Some(1i32));
```

### オブジェクトのデシリアライズとシリアライズ

* [serde](https://docs.serde.rs/serde/)クレートを使用して、オブジェクトをシリアライズとデシリアライズ
* [axum](https://docs.rs/axum/latest/axum/)クレートは、`json`フィーチャーを有効にすることで、JSONのシリアライズとデシリアライズをサポート
* フィールド名の変換が必要なフィールドには、`#[serde(rename = "new_name")]`属性を付与
* フィールド名をキャメルケースで扱う場合は、`#[serde(rename_all = "camelCase")]`属性を構造体に付与

### ロギング

* [tracing](https://docs.rs/tracing/latest/tracing/)クレートを使用して、ログ出力
* [env_filter](https://docs.rs/env_filter/0.1.2/env_filter/)を使用して、環境変数などからログベレルを取得
* `tracing-subscriber`クレートで`std`と`env-filter`フィーチャを有効にして、ログレベルを指定
* [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)クレートで、`std`と`env-filter`フィーチャを有効にして、ログをJSON形式で出力

```rust
// ログの出力形式を設定
let subscriber = tracing_subscriber::fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_target(false)
    .json();

tracing_subscriber::registry()
    .with(subscriber)
    .with(env_filter)
    .with(opentelemetry)
    .try_init()?;
```

* リクエストハンドラに`#[tracing::instrument]`属性を付与することで、リクエストハンドラの呼び出し時と終了時にログを出力
* `tower`のレイヤを利用して、リクエストを受けたときとレスポンスを返すときにログを出力

```rust
let app = router
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        );
```

### モック

* [mockall](https://docs.rs/mockall/latest/mockall/)クレートを使用して、トレイトや構造体のモック（テストダブル）を作成
* データベースを使用した統合テストでは、スタブを使用して、データベースからの読み込みを模倣して、モックを使用してデータベースへの書き込みを模倣

### グレースフルシャットダウン

* [opentelemetry](https://docs.rs/opentelemetry/latest/opentelemetry/)クレートを使用して、`CTRL+C`や`SIGTERM`などのシグナルを受信したときに、すべてのログを出力してから、アプリケーションを終了
* 本書274ページを参照

### SPAとの統合

* 開発時は、`CORS（オリジン間リソース共有）`と次の通り有効にする

```rust
fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

// ルーターを登録
let app = router.layer(cors()).with_state(registry);
```

### OpenAPI

* [utoipa](https://docs.rs/utoipa/latest/utoipa/)クレートを使用して、OpenAPIスキーマを生成

### PostgreSQLのトランザクション

* リードコミット: **SELECTが実行される直前**までにコミットされたデータを参照
* リピータブルリード: **トランザクションが開始される直前**までにコミットされたデータを参照
* シリアライザブル: それぞれのトランザクションの各命令が、トランザクションをまたがって実行した場合、どの順番で実行しても同じ結果になる（直列化）ことを保証
* PostgreSQLのシリアライザブルは、トランザクションが開始される直前のスナップショットを読み込む
* 読み込みをロックしたい場合は、シリアライザブルを使用して、直列化異常異常の発生に対処

### Docker

* マルチステージビルドを使用して、アプリケーションをビルドするコンテナと、アプリを実行するコンテナを分けることで、ビルド性能を向上
