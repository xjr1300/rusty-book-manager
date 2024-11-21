///
/// `tokio::main`マクロを展開する。
///
/// ```sh
/// cargo expand --example async_send_get_requests_in_tokio_main
/// ```
///
/// ```rust
/// #![feature(prelude_import)]
/// #[prelude_import]
/// use std::prelude::rust_2021::*;
/// #[macro_use]
/// extern crate std;
/// fn main() {
///     let body = async {
///         let req1 = reqwest::get("https://www.google.com/");
///         let req2 = reqwest::get("https://doc.rust-lang.org/std/");
///         let parallel_reqs = async move { futures::future::join(req1, req2).await };
///         let (result1, result2) = parallel_reqs.await;
///         {
///             ::std::io::_print(format_args!("status1: {0}\n", result1.unwrap().status()));
///         };
///         {
///             ::std::io::_print(format_args!("status2: {0}\n", result2.unwrap().status()));
///         };
///     };
///     #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
///     {
///         return tokio::runtime::Builder::new_multi_thread()
///             .enable_all()
///             .build()
///             .expect("Failed building the Runtime")
///             .block_on(body);
///     }
/// }
/// ```
#[tokio::main]
async fn main() {
    // GETリクエストを送信する仕組みを構築
    let req1 = reqwest::get("https://www.google.com/");
    let req2 = reqwest::get("https://doc.rust-lang.org/std/");

    // 2つのGETリクエストを並列で実行する仕組みを構築
    let parallel_reqs = async move { futures::future::join(req1, req2).await };

    // 非同期ランタイム上で2つのGETリクエストを並列で送信
    // 非同期ランタイム上で、GETリクエストを送信する準備ができたら、GETリクエストが実際に送信される。
    let (result1, result2) = parallel_reqs.await;

    println!("status1: {}", result1.unwrap().status());
    println!("status2: {}", result2.unwrap().status());
}
