// 重导出整理路径
// 视频定位：P09 04:21 · 运行：cargo run -p ch09 --example p09_s16
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 把深层路径重导出成扁平的对外名字
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn main() {}
