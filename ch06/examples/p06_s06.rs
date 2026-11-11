// & 不取得所有权
// 视频定位：P06 01:49 · 运行：cargo run -p ch06 --example p06_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{s1} 的长度是 {len}");   // s1 仍可用
}
