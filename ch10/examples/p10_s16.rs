// 不能按下标取
// 视频定位：P10 04:19 · 运行：cargo run -p ch10 --example p10_s16
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let s = String::from("你好 Rust");
    println!("字节数：{}", s.len());
    for b in s.bytes() {
        print!("{b:02x} ");
    }
    println!();
    for c in s.chars() {
        print!("[{c}]");
    }
    println!();
    println!("切片 &s[0..3] = {}", &s[0..3]);
}
