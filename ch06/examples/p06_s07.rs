// 点号帮你省了星号
// 视频定位：P06 02:04 · 运行：cargo run -p ch06 --example p06_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn show(s: &String) {
    println!("{}", s.len());       // 自动解引用
    println!("{}", (*s).len());    // 等价的显式写法
}

fn main() {
    let s = String::from("hello");
    show(&s);
}
