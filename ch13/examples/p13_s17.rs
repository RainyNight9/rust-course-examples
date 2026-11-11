// 字段是引用时
// 视频定位：P13 05:08 · 运行：cargo run -p ch13 --example p13_s17
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("很久以前…");
    let first = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first };
}
