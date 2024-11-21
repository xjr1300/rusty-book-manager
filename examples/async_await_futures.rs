async fn a1() -> i32 {
    1
}

async fn a2() -> i32 {
    2
}

async fn ans(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
async fn main() {
    let a1 = a1().await;
    let a2 = a2().await;
    let ans = ans(a1, a2).await;
    println!("{}", ans);
}
