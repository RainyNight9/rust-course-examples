// 只借用到的那个
// 视频定位：P16 02:01 · 运行：cargo run -p ch16 --example p16_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct State { name: String, count: i32 }

fn main() {
    let mut s = State { name: String::from("a"), count: 0 };
    let mut bump = || s.count += 1;   // 只可变借用 count
    let label = &s.name;              // 仍可不可变借用 name
    bump();
    println!("{label}");
}
