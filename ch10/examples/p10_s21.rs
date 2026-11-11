// entry 少查一次
// 视频定位：P10 05:42 · 运行：cargo run -p ch10 --example p10_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::collections::HashMap;

fn main() {
    let text = "hello world hello";
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{counts:?}");
    println!("hello 出现 {:?} 次", counts.get("hello"));
}
