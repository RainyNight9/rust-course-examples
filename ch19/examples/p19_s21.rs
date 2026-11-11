// AtomicUsize 更轻
// 视频定位：P19 05:47 · 运行：cargo run -p ch19 --example p19_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {

    let hit = Arc::new(AtomicUsize::new(0));
    let mut hs = vec![];
    for _ in 0..8 {
        let h = Arc::clone(&hit);
        hs.push(thread::spawn(move || {
            h.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for h in hs { h.join().unwrap(); }
    println!("共 {} 次", hit.load(Ordering::Relaxed));
}
