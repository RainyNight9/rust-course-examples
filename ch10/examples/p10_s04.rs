// 三种集合一起用
// 视频定位：P10 01:11 · 运行：cargo run -p ch10 --example p10_s04
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::collections::HashMap;

fn main() {
    let mut nums = vec![1, 2, 3];
    nums.push(4);

    let mut text = String::from("Rust");
    text.push_str(" 集合");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    println!("{nums:?} / {text} / {:?}", scores.get("Blue"));
}
