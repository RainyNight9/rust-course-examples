// 遍历不靠手写下标
// 视频定位：P04 06:13 · 运行：cargo run -p ch04 --example p04_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    for i in 1..4 {
        println!("{i}");
    }
    for (idx, ch) in "abc".chars().enumerate() {
        println!("{idx}: {ch}");
    }
    for i in (1..6).rev().step_by(2) {
        println!("{i}");
    }
}
