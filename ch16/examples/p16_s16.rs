// 不消费一行不跑
// 视频定位：P16 04:41 · 运行：cargo run -p ch16 --example p16_s16
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]



fn main() {
    let v = vec![1, 2, 3];
    let it = v.iter().map(|x| {
        println!("处理 {x}");     // 不 collect 就永远不打印
        x * 2
    });
    println!("只搭好管道");
    let out: Vec<_> = it.collect();   // 到这里才真正执行
}
