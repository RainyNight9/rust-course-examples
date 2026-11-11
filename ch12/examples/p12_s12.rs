// 编译期各生成一份
// 视频定位：P12 03:13 · 运行：cargo run -p ch12 --example p12_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn identity<T>(x: T) -> T { x }

fn main() {
    let a = identity(5i32);     // 生成一份 identity_i32
    let b = identity("hi");     // 再生成一份 identity_str
    println!("{a} {b}");
}

// 单态化等价于手写两份：
// fn identity_i32(x: i32) -> i32 { x }
// fn identity_str(x: &str) -> &str { x }
