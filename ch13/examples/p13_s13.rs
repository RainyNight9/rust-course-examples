// 改数据归属别加标注
// 视频定位：P13 03:54 · 运行：cargo run -p ch13 --example p13_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn owned() -> String {
    let s = String::from("临时");
    s
}

fn literal() -> &'static str {
    "常量"
}

fn main() {
    println!("{} {}", owned(), literal());
}
