// channel 有发送端和接收端
// 视频定位：P19 03:21 · 运行：cargo run -p ch19 --example p19_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::sync::mpsc;
use std::thread;

fn main() {

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(String::from("hi")).unwrap();
    });

    let msg = rx.recv().unwrap();   // 阻塞等待
    println!("收到 {msg}");
}
