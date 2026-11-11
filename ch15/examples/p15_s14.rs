// 别用 is_ok 糊弄
// 视频定位：P15 04:09 · 运行：cargo run -p ch15 --example p15_s14
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::env;

fn main() {

    match env::var("CASE_INSENSITIVE") {
        Ok(v) => println!("开关值 {v}"),
        Err(env::VarError::NotPresent) => println!("未设置"),
        Err(env::VarError::NotUnicode(_)) => eprintln!("值不是 UTF-8"),
    }
}
