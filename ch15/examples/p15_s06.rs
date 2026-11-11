// 遇到乱码会直接崩
// 视频定位：P15 01:43 · 运行：cargo run -p ch15 --example p15_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::env;

fn main() {

    // env::args() 遇到非 UTF-8 参数会 panic
    let raw: Vec<_> = env::args_os().collect();
    for a in &raw {
        println!("{a:?}");   // OsString 可无损保存
    }
}
