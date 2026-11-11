// cfg(test) 的意义
// 视频定位：P14 04:45 · 运行：cargo run -p ch14 --example p14_s15
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() { assert_eq!(internal_adder(2, 2), 4); }
}

fn main() {}
