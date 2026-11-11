// 一段最小的 Rust 程序
// 视频定位：P01 01:20 · 运行：cargo run -p ch01 --example p01_s04
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let greeting = "Hello, Rust";
    let mut count = 1;
    count += 1;
    println!("{} / {}", greeting, count);
}
