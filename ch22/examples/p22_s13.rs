// 把行为与代码当参数
// 视频定位：P22 03:39 · 运行：cargo run -p ch22 --example p22_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn add_one(x: i32) -> i32 { x + 1 }
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => { /* ... */ };
}

fn main() {
    do_twice(add_one, 5);

}
