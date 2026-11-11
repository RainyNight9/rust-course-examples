// 每行都在重新分配
// 视频定位：P15 05:06 · 运行：cargo run -p ch15 --example p15_s17
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();   // 只转一次
    contents.lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

fn main() {}
