// ? 不是哪里都能写
// 视频定位：P11 04:44 · 运行：cargo run -p ch11 --example p11_s17
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;
    Ok(())
}
