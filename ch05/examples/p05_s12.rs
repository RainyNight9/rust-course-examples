// 在最后一个使用点之后
// 视频定位：P05 03:29 · 运行：cargo run -p ch05 --example p05_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    let outer = String::from("out");
    {
        let inner = String::from("in");
        println!("{inner}");
    } // inner 在这里 drop
    println!("{outer}");
} // outer 在这里 drop
