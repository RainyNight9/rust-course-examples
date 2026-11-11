// 想改，三处都写 mut
// 视频定位：P06 02:47 · 运行：cargo run -p ch06 --example p06_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {

    let mut s = String::from("hello");
    change(&mut s);
}
