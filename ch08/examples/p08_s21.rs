// 加变体后静默漏掉
// 视频定位：P08 05:38 · 运行：cargo run -p ch08 --example p08_s21
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

enum Status { Running, Failed(String) }

fn main() {
    // 需求变了：状态多出 Failed
    let s = Status::Failed(String::from("超时"));
    // 只关心 Running，新增的 Failed 会被静默忽略
    if let Status::Running = s {
        println!("正在运行");
    }
    // 换成 match 漏分支就会编译失败
    println!("程序照常结束，失败状态被漏掉了");
}
