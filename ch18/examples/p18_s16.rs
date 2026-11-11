// 冲突会直接崩溃
// 视频定位：P18 04:42 · 运行：cargo run -p ch18 --example p18_s16
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::cell::RefCell;

fn main() {
    let x = RefCell::new(vec![1, 2, 3]);
    let a = x.borrow();
    assert!(x.try_borrow_mut().is_err());   // 不崩溃的写法
    drop(a);                                // 及时归还
    if let Ok(mut v) = x.try_borrow_mut() {
        v.push(4);
    }
    assert_eq!(*x.borrow(), vec![1, 2, 3, 4]);
}
