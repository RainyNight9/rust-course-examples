// len 不是容量
// 视频定位：P10 01:40 · 运行：cargo run -p ch10 --example p10_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(3);
    println!("初始 len={} cap={}", v.len(), v.capacity());
    for i in 1..=4 {
        v.push(i);
        println!("push {i} 后 len={} cap={}", v.len(), v.capacity());
    }
}
