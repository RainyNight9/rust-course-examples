// block_on 循环 poll
// 视频定位：P20 03:01 · 运行：cargo run -p ch20 --example p20_s11
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct Noop;
impl Wake for Noop {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = Waker::from(Arc::new(Noop));
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn main() {
    println!("{}", block_on(async { 40 + 2 }));
}
