// 零成本抽象不是口号
// 视频定位：P01 04:13 · 运行：cargo run -p ch01 --example p01_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let total: i64 = (1..=100)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("sum of squares of even numbers 1..=100: {}", total);
}
