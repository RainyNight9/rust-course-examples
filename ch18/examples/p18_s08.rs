// 把数据放到堆上
// 视频定位：P18 02:27 · 运行：cargo run -p ch18 --example p18_s08
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let b = Box::new(5);
    println!("{b}");   // 自动解引用
}
