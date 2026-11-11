// vec! / push / pop
// 视频定位：P10 01:58 · 运行：cargo run -p ch10 --example p10_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);

    let v2 = vec![1, 2, 3];
    let last = v.pop();      // Option<i32>
}
