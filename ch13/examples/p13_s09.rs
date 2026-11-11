// 最后一次使用为止
// 视频定位：P13 02:41 · 运行：cargo run -p ch13 --example p13_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("{first}");   // 最后一次使用
    v.push(4);             // 此处借用已经结束，合法
}
