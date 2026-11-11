// 一份数据多个所有者
// 视频定位：P18 04:27 · 运行：cargo run -p ch18 --example p18_s15
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::rc::Rc;
enum List { Cons(i32, Rc<List>), Nil }

fn main() {

    let a = Rc::new(List::Cons(5, Rc::new(List::Nil)));
    let b = List::Cons(10, Rc::clone(&a));
    let c = List::Cons(15, Rc::clone(&a));

    println!("计数 = {}", Rc::strong_count(&a));
}
