// 编译期不管，运行时拦
// 视频定位：P04 04:06 · 运行：cargo run -p ch04 --example p04_s14
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    // println!("{}", a[index]);     // 越界会 panic
    println!("{:?}", a.get(index));  // None，不崩溃
}
