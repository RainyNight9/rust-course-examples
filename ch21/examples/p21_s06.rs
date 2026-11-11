// 运行时动态分发
// 视频定位：P21 01:43 · 运行：cargo run -p ch21 --example p21_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub trait Draw { fn draw(&self); }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for c in &self.components { c.draw(); }
    }
}

fn main() {}
