// map / filter / take
// 视频定位：P16 04:28 · 运行：cargo run -p ch16 --example p16_s15
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .filter(|x| *x > 4)
        .take(2)
        .collect();
}
