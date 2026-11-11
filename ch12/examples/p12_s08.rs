// 结构体和枚举也能带参数
// 视频定位：P12 02:10 · 运行：cargo run -p ch12 --example p12_s08
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    struct Point<T, U> { x: T, y: U }

    let a = Point { x: 5, y: 10 };
    let b = Point { x: 1, y: 2.5 };

    impl<T> Point<T, T> {
        fn x(&self) -> &T { &self.x }
    }
}
