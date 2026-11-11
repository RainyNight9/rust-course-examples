// Arc<Mutex<T>>
// 视频定位：P19 04:26 · 运行：cargo run -p ch19 --example p19_s16
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("结果：{}", *counter.lock().unwrap());
}
