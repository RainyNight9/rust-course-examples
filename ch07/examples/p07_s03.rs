// 元组藏不住的顺序
// 视频定位：P07 00:56 · 运行：cargo run -p ch07 --example p07_s03
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn main() {
    // 30 是宽还是高？只能靠注释和记忆
    let rect = (30, 50);
    let area = rect.0 * rect.1;
    println!("面积 {area}");
}
