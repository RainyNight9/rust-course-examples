// 每次都要还回来？
// 视频定位：P06 00:53 · 运行：cargo run -p ch06 --example p06_s03
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)      // 不得不把 s 还回去
}

fn main() {}
