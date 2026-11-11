// println! 的占位符不止一种
// 视频定位：P02 01:54 · 运行：cargo run -p ch02 --example p02_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let name = "Rust";
    let year = 2024;
    println!("Hello, {}!", name);
    println!("{0} {1}, {0} again", name, year);
    println!("edition = {year}");
    println!("debug: {:?}", vec![1, 2, 3]);
}
