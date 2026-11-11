// 不用再还回来了
// 视频定位：P06 01:05 · 运行：cargo run -p ch06 --example p06_s04
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 是引用，不拥有值，这里不会 drop 数据

fn main() {
    let s = String::from("hello");
    let n = calculate_length(&s);
    println!("{s} 有 {n} 个字节");   // s 仍可用
}
