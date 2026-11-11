// 进函数、出函数都一样
// 视频定位：P05 06:33 · 运行：cargo run -p ch05 --example p05_s22
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn consume(s: String) -> usize { s.len() }

fn main() {
    let s = String::from("hello");
    let n = consume(s);       // s 被移动进函数
    // println!("{s}");       // 这里再用会报 E0382
    println!("长度 {n}");
}
