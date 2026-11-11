// 能捕获环境的匿名函数
// 视频定位：P16 01:30 · 运行：cargo run -p ch16 --example p16_s05
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;   // 捕获了 x

    let add_one = |x: i32| -> i32 { x + 1 };
    println!("{}", equal_to_x(4));  // true
}
