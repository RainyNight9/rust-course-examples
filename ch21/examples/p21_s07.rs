// impl Trait 与 dyn 的账
// 视频定位：P21 01:56 · 运行：cargo run -p ch21 --example p21_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

trait Draw { fn draw(&self) -> String; }
struct Button;
impl Draw for Button {
    fn draw(&self) -> String { String::from("button") }
}

// 静态分发：为每个具体类型生成一份代码
fn render_static<T: Draw>(item: &T) -> String { item.draw() }

// 动态分发：擦掉具体类型，走虚表
fn render_dyn(item: &dyn Draw) -> String { item.draw() }

fn main() {
    println!("{} {}", render_static(&Button), render_dyn(&Button));
}
