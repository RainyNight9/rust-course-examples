// 把检查挪到运行时
// 视频定位：P18 05:19 · 运行：cargo run -p ch18 --example p18_s18
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::cell::RefCell;

fn main() {

    let x = RefCell::new(5);
    let a = x.borrow();
    let b = x.borrow();        // 多个不可变借用，OK
    // let c = x.borrow_mut(); // 运行时 panic：already borrowed
    drop(a); drop(b);
    *x.borrow_mut() += 1;
}
