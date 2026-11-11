// macro_rules! 的重复
// 视频定位：P22 03:54 · 运行：cargo run -p ch22 --example p22_s14
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

#[macro_export]
macro_rules! sum {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut total = 0;
            $( total += $x; )*
            total
        }
    };
}

fn main() {
    println!("{}", sum!(1, 2, 3, 4));
}
