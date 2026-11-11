// 模式的三种进阶
// 视频定位：P08 04:41 · 运行：cargo run -p ch08 --example p08_s17
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: -3 };
    match msg {
        Message::Move { x, .. } if x > 0 => println!("向右 {x}"),
        Message::Move { .. } => println!("没向右"),
        Message::Write(s) => println!("文本 {s}"),
        Message::Quit => println!("退出"),
    }
    let note = Message::Write(String::from("hi"));
    println!("是 Write 吗：{}", matches!(note, Message::Write(_)));
}
