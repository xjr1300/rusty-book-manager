use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::executor::block_on;
use futures::future::FutureExt;

struct Number {
    val: i32,
}

impl Future for Number {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.val)
    }
}

fn a1() -> impl Future<Output = i32> {
    Number { val: 1 }
}

fn a2() -> impl Future<Output = i32> {
    Number { val: 2 }
}

fn ans(a: i32, b: i32) -> impl Future<Output = i32> {
    Number { val: a + b }
}

fn main() {
    let ans = block_on(a1().then(|a| a2().then(move |b| ans(a, b))));
    println!("{}", ans);
}
