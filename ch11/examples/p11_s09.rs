// Ok 或 Err
// 视频定位：P11 02:36 · 运行：cargo run -p ch11 --example p11_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::fs::File;

fn main() {

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("打开文件失败：{error:?}"),
    };
}
