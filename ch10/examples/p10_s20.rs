// insert / get / entry
// 视频定位：P10 05:29 · 运行：cargo run -p ch10 --example p10_s20
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    if let Some(score) = scores.get("Blue") {
        println!("{score}");
    }

    let count = scores.entry(String::from("Red")).or_insert(0);
    *count += 1;
}
