// 传参用 impl Fn，存起来用装箱
// 视频定位：P16 01:41 · 运行：cargo run -p ch16 --example p16_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {

    let add = |x| x + 1;
    assert_eq!(apply_twice(add, 1), 3);

    let fs: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
    ];
}
