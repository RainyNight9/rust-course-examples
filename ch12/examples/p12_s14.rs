// 默认实现可以覆盖
// 视频定位：P12 03:44 · 运行：cargo run -p ch12 --example p12_s14
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub trait Summary {
    fn summarize(&self) -> String;

    fn hint(&self) -> String {
        String::from("（阅读更多…）")
    }
}

pub struct Tweet { pub content: String }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@alice: {}", self.content)
    }
    // hint 不写，用默认实现
}

fn main() {
    let t = Tweet { content: String::from("hello") };
    println!("{} / {}", t.summarize(), t.hint());
}
