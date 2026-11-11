// push_str / + / format!
// 视频定位：P10 04:39 · 运行：cargo run -p ch10 --example p10_s17
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;      // s1 被移动，s2 仍可用

    let s4 = format!("{s3}-{s2}");
}
