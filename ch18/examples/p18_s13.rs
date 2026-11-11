// Rc 不能跨线程
// 视频定位：P18 03:48 · 运行：cargo run -p ch18 --example p18_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("共享"));
    println!("{}", Rc::strong_count(&a));     // 1
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    println!("{}", Rc::strong_count(&a));     // 3
    drop(c);
    println!("{} {}", Rc::strong_count(&a), b); // 2
}
