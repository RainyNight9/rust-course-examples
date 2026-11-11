// 深拷贝默认不做
// 视频定位：P05 02:22 · 运行：cargo run -p ch05 --example p05_s08
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();   // 显式深拷贝堆数据
    println!("s1 = {s1}, s2 = {s2}");
}
