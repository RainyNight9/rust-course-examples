// 让析构保持沉默
// 视频定位：P18 05:33 · 运行：cargo run -p ch18 --example p18_s19
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::rc::Rc;
use std::cell::RefCell;

struct Node { next: RefCell<Option<Rc<Node>>> }
impl Drop for Node {
    fn drop(&mut self) { println!("Node 被释放"); }
}

fn main() {
    let a = Rc::new(Node { next: RefCell::new(None) });
    let b = Rc::new(Node { next: RefCell::new(Some(Rc::clone(&a))) });
    *a.next.borrow_mut() = Some(Rc::clone(&b));   // 成环
    println!("计数 {}", Rc::strong_count(&a));
}
