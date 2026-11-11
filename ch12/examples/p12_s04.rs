// 推断不了就显式写
// 视频定位：P12 01:08 · 运行：cargo run -p ch12 --example p12_s04
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nums = vec![34, 50, 25];
    let chars = vec!['y', 'm', 'a'];
    println!("{}", largest(&nums));
    println!("{}", largest::<char>(&chars));
}
