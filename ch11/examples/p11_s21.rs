// 把决定权交回去
// 视频定位：P11 05:40 · 运行：cargo run -p ch11 --example p11_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::fs;
use std::io;

pub fn read_config(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;
    if text.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "配置内容为空"));
    }
    Ok(text)
}

fn main() {
    match read_config("config.txt") {
        Ok(text) => println!("读到 {} 字节", text.len()),
        Err(e) => eprintln!("读配置失败：{e}"),
    }
}
