// 引用会一路降级
// 视频定位：P18 02:08 · 运行：cargo run -p ch18 --example p18_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn takes_str(s: &str) { println!("{s}"); }

fn main() {
    let boxed = Box::new(String::from("hello"));
    takes_str(&boxed);       // &Box<String> 一路转成 &str
    let n = Box::new(5);
    assert_eq!(*n + 1, 6);   // 用星号取出 i32
}
