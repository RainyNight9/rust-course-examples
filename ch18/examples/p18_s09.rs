// trait 对象动态派发
// 视频定位：P18 02:41 · 运行：cargo run -p ch18 --example p18_s09
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

trait Draw { fn draw(&self) -> String; }
struct Circle;
struct Square;
impl Draw for Circle { fn draw(&self) -> String { String::from("圆") } }
impl Draw for Square { fn draw(&self) -> String { String::from("方") } }

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle), Box::new(Square),
    ];
    for s in &shapes { println!("{}", s.draw()); }
}
