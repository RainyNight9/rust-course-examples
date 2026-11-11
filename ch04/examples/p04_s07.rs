// 同名，却是新的绑定
// 视频定位：P04 01:59 · 运行：cargo run -p ch04 --example p04_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    let mut x = 5;
    // x = "five";   // 错误：mut 不能换类型
    println!("{spaces} {x}");
}
