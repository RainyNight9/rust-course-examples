// 同名函数两套实现
// 视频定位：P14 03:59 · 运行：cargo run -p ch14 --example p14_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

#[cfg(test)]
fn now() -> u64 { 0 }

#[cfg(not(test))]
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {}
