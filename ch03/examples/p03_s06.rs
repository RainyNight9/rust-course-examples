// &mut 第一次出现
// 视频定位：P03 01:54 · 运行：cargo run -p ch03 --example p03_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::io;

fn main() {
    println!("猜一个 1 到 100 之间的数字：");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("读取输入失败");
    println!("你猜的是：{guess}");
}
