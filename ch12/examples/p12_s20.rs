// where 让签名清爽
// 视频定位：P12 05:15 · 运行：cargo run -p ch12 --example p12_s20
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::fmt::{Debug, Display};

fn compare_and_print<T, U>(t: &T, u: &U) -> bool
where
    T: Display + PartialOrd,
    U: Debug + Clone,
{
    println!("t = {t}, u = {u:?}");
    true
}

fn main() {
    let result = compare_and_print(&5, &vec![1, 2, 3]);
    println!("{result}");
}
