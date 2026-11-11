// 定义共同行为
// 视频定位：P12 03:34 · 运行：cargo run -p ch12 --example p12_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub trait Summary {
    fn summarize(&self) -> String;

    fn default_hint(&self) -> String {
        String::from("（阅读更多…）")   // 默认实现
    }
}

fn main() {}
