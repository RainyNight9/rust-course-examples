// 右花括号那里自动释放
// 视频定位：P05 03:17 · 运行：cargo run -p ch05 --example p05_s11
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    {
        let s = String::from("hello");
        println!("{s}");
    }   // 作用域结束 → 自动调用 drop
}
