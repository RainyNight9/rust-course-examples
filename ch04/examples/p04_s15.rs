// 参数和返回值都要标类型
// 视频定位：P04 04:26 · 运行：cargo run -p ch04 --example p04_s15
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let n = plus_one(5);
    println!("{n}");   // 6
}
