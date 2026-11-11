// 元组和数组
// 视频定位：P04 03:52 · 运行：cargo run -p ch04 --example p04_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;          // 解构
    let five_hundred = tup.0;

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
}
