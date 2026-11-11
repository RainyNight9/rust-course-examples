// 别在库里直接崩
// 视频定位：P15 02:38 · 运行：cargo run -p ch15 --example p15_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::error::Error;
use std::fs;

fn read(path: &str) -> Result<String, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}

fn main() {}
