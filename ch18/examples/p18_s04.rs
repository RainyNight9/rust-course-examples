// 只有一个指针宽
// 视频定位：P18 01:16 · 运行：cargo run -p ch18 --example p18_s04
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::mem::size_of;

fn main() {
    println!("{}", size_of::<Box<i32>>());   // 64 位平台上是 8
    println!("{}", size_of::<i32>());        // 4
    let b = Box::new([0u8; 1024]);
    println!("{}", std::mem::size_of_val(&b));   // 仍是 8
}
