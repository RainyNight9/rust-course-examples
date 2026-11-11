// 期望消息要写准
// 视频定位：P14 02:46 · 运行：cargo run -p ch14 --example p14_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn checked_get(v: &[i32], i: usize) -> i32 {
    if i >= v.len() { panic!("下标越界：{i}"); }
    v[i]
}

#[test]
#[should_panic(expected = "下标越界")]
fn out_of_range() {
    checked_get(&[1, 2, 3], 99);
}

fn main() {}
