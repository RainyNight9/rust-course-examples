// 忘了 drop(tx) 会挂住
// 视频定位：P19 03:47 · 运行：cargo run -p ch19 --example p19_s14
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel::<i32>();
    let tx2 = tx.clone();
    thread::spawn(move || { tx2.send(7).unwrap(); });
    drop(tx); // 少了这一行，下面的 for 会永远等下去

    for v in rx { println!("收到 {v}"); }
    // 诊断死等改用：rx.recv_timeout(Duration::from_millis(500))
}
