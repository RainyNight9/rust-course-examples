// next 是唯一必需方法
// 视频定位：P16 03:29 · 运行：cargo run -p ch16 --example p16_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let v1 = vec![1, 2, 3];
    let mut iter = v1.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));

    for val in &v1 { println!("{val}"); }   // for 循环本身就是语法糖
}
