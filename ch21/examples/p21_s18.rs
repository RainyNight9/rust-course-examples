// if 里串起模式
// 视频定位：P21 04:54 · 运行：cargo run -p ch21 --example p21_s18
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn handle(input: Option<&str>) {
    // 2024 Edition（Rust 1.88+）起 let 链稳定
    if let Some(s) = input
        && let Ok(n) = s.parse::<i32>()
        && n > 0
    {
        println!("正整数 {n}");
    }
    println!("匹配吗 {}", matches!(input, Some("ok")));
}

fn main() {}
