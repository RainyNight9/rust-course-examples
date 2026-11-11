// impl 块里写方法
// 视频定位：P07 04:11 · 运行：cargo run -p ch07 --example p07_s15
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {


    let rect = Rectangle { width: 30, height: 50 };
    rect.area();
}
