// 强制转移所有权
// 视频定位：P16 03:17 · 运行：cargo run -p ch16 --example p16_s11
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::thread;

fn main() {

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{v:?}");   // v 的所有权已进闭包
    });
    handle.join().unwrap();
}
