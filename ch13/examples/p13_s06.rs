// 输出跟着谁要写清
// 视频定位：P13 01:47 · 运行：cargo run -p ch13 --example p13_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    let _ = y;
    x
}

fn main() {
    let s = String::from("hello world");
    let word = first(&s, "unused");
    println!("{word}");
}
