// 自定义解引用行为
// 视频定位：P18 03:21 · 运行：cargo run -p ch18 --example p18_s11
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T { &self.0 }
    }

    let x = MyBox(String::from("hello"));
    fn hello(s: &str) { println!("{s}"); }
    hello(&x);     // &MyBox<String> → &String → &str
}
