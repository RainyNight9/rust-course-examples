// 索引 vs get
// 视频定位：P10 02:44 · 运行：cargo run -p ch10 --example p10_s10
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];               // 3，越界会 panic
    match v.get(2) {
        Some(x) => println!("{x}"),
        None    => println!("没有这个下标"),
    }
}
