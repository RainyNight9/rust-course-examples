// Copy 和 Move 摆一起看
// 视频定位：P05 06:55 · 运行：cargo run -p ch05 --example p05_s23
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn takes_int(x: i32) { println!("{x}"); }
fn takes_str(s: String) { println!("{s}"); }

fn main() {
    let x = 5;
    takes_int(x);
    println!("{x}");            // OK：i32 是 Copy
    let s = String::from("hi");
    takes_str(s);
    // println!("{s}");         // 报错：String 已移动
}
