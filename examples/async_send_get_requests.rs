fn main() {
    // GETリクエストを送信する仕組みを構築
    let req1 = reqwest::get("https://www.google.com/");
    let req2 = reqwest::get("https://doc.rust-lang.org/std/");

    // 2つのGETリクエストを並列で実行する仕組みを構築
    let parallel_reqs = async move { futures::future::join(req1, req2).await };

    // tokio非同期ランタイムを起動
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // 非同期ランタイム上で2つのGETリクエストを並列で送信
    // 非同期ランタイム上で、GETリクエストを送信する準備ができたら、GETリクエストが実際に送信される。
    let (result1, result2) = rt.block_on(parallel_reqs);

    println!("status1: {}", result1.unwrap().status());
    println!("status2: {}", result2.unwrap().status());
}
