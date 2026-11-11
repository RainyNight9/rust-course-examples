// panic 让锁中毒
// 视频定位：P19 04:40 · 运行：cargo run -p ch19 --example p19_s17
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let m = Arc::new(Mutex::new(vec![1]));
    let m2 = Arc::clone(&m);
    let h = thread::spawn(move || {
        let mut g = m2.lock().unwrap();
        g.push(2);
        panic!("故意崩");
    });
    let _ = h.join(); // 线程 panic，锁被标记为中毒

    match m.lock() {
        Ok(g) => println!("正常 {g:?}"),
        Err(e) => println!("中毒，仍能取回 {:?}", e.into_inner()),
    }
}
