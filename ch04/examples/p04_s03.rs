// let 与 mut
// 视频定位：P04 00:52 · 运行：cargo run -p ch04 --example p04_s03
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let x = 5;
    // x = 6;   // 编译错误：不能给不可变变量赋值两次

    let mut y = 5;
    y = 6;      // OK
}
