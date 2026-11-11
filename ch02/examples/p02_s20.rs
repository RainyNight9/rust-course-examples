// 惯用法交给 clippy 盯
// 视频定位：P02 05:42 · 运行：cargo run -p ch02 --example p02_s20
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let flag = true;
    if flag == true {
        println!("on");
    }
    let v: Vec<i32> = vec![1, 2, 3];
    if v.len() == 0 {
        println!("empty");
    }
}
