// 参数写 &str 更好
// 视频定位：P06 06:25 · 运行：cargo run -p ch06 --example p06_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let owned = String::from("hello world");
    println!("{}", first_word(&owned));    // &String 转成 &str
    println!("{}", first_word("hi rust")); // 字面量本身就是 &str
}
