// 取完任务先放锁
// 视频定位：P22 05:02 · 运行：cargo run -p ch22 --example p22_s18
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn worker_loop(rx: Arc<Mutex<Receiver<Job>>>) {
    loop {
        // 锁住、取出一个任务，语句结束守护即释放
        let job = rx.lock().unwrap().recv();
        let job = match job {
            Ok(job) => job,
            Err(_) => break,   // 发送端关闭，退出循环
        };
        job();                 // 锁外执行，别的 worker 才能同时干活
    }
}

// 反例：while let Ok(job) = rx.lock().unwrap().recv() { job(); }
// 守护活过整个循环体，所有 worker 被串行化

fn main() {}
