// 返回的是 impl Future
// 视频定位：P20 01:53 · 运行：cargo run -p ch20 --example p20_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

// 下面两种写法等价
async fn answer() -> u32 { 42 }

fn answer_explicit() -> impl std::future::Future<Output = u32> {
    async { 42 }
}

// 2024 版中 trait 里的 async fn 走 RPITIT，返回匿名 future

fn main() {}
