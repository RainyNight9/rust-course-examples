// 引用一段数据
// 视频定位：P06 06:13 · 运行：cargo run -p ch06 --example p06_s20
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];     // &str
    let world = &s[6..11];

}
