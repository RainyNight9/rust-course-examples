// 闭包要满足Send 与 'static
// 视频定位：P19 01:43 · 运行：cargo run -p ch19 --example p19_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

// std::thread::spawn 的约束（简化）：
//   F: FnOnce() -> T + Send + 'static
//   T: Send + 'static
use std::thread;

fn main() {
    let owned = String::from("hi");
    let h: thread::JoinHandle<usize> = thread::spawn(move || {
        println!("{owned}");
        owned.len()
    });
    println!("拿到 {}", h.join().unwrap());
}
