// 把范围收窄
// 视频定位：P17 03:29 · 运行：cargo run -p ch17 --example p17_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub mod api {
    pub fn public_fn() {}
    pub(crate) fn inside_fn() {}
    fn private_fn() {}
}

pub use api::public_fn;

fn main() {}
