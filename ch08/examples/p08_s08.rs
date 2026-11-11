// 四种变体构造一遍
// 视频定位：P08 02:14 · 运行：cargo run -p ch08 --example p08_s08
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 3, y: 4 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 128);
    println!("{m1:?} / {m2:?} / {m3:?} / {m4:?}");
}
