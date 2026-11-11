// expect 底下是Result
// 视频定位：P03 02:12 · 运行：cargo run -p ch03 --example p03_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::io;

fn main() {
    let mut guess = String::new();
    let n = io::stdin()
        .read_line(&mut guess)
        .expect("读取失败");
    println!("读了 {n} 个字节：{guess:?}");
}
