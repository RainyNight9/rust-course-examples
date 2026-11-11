// fold / any / zip
// 视频定位：P16 05:32 · 运行：cargo run -p ch16 --example p16_s19
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let v = vec![1, 2, 3, 4];
    let total = v.iter().fold(0, |acc, x| acc + x);
    let has_even = v.iter().any(|x| x % 2 == 0);
    let pairs: Vec<_> = v.iter().zip(['a', 'b', 'c', 'd']).collect();
    let squares: Vec<_> = v.iter().map(|x| x * x).take(2).collect();
}
