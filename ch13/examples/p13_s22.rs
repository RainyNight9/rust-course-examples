// 泛型 + trait + 生命周期
// 视频定位：P13 06:41 · 运行：cargo run -p ch13 --example p13_s22
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告：{ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {}
