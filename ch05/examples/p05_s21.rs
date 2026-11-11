// 函数会拿走所有权
// 视频定位：P05 06:18 · 运行：cargo run -p ch05 --example p05_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn takes_ownership(s: String) { println!("{s}"); }
fn makes_copy(x: i32)         { println!("{x}"); }

fn main() {

    let s = String::from("hello");
    takes_ownership(s);   // s 被移动
    let x = 5;
    makes_copy(x);        // x 仍可用
}
