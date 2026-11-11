// 两种构建溢出不同
// 视频定位：P04 03:15 · 运行：cargo run -p ch04 --example p04_s11
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    // 用 black_box 阻止编译期常量求值：debug 下运行时 panic，release 下回绕到 0
    let a: u8 = std::hint::black_box(255);
    // debug 构建下这一行会 panic
    let b = a + 1;
    println!("{b}");
    println!("{:?}", a.checked_add(1));
}
