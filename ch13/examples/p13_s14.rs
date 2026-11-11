// 冒号读作不少于
// 视频定位：P13 04:12 · 运行：cargo run -p ch13 --example p13_s14
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn choose<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,
{
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s = String::from("abcd");
    println!("{}", choose("ab", &s));
}
