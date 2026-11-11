// no_run 只编译不跑
// 视频定位：P17 02:40 · 运行：cargo run -p ch17 --example p17_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

/// 把两个整数相加。
///
/// # Panics
///
/// 当前实现不会 panic。
///
/// # Examples
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
///
/// ```no_run
/// let _ = std::fs::read_to_string("/etc/hosts");
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }

fn main() {}
