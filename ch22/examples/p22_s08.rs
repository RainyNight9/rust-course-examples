// 长度写错照样编译
// 视频定位：P22 02:12 · 运行：cargo run -p ch22 --example p22_s08
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn bad_slice(data: &[u8]) -> &[u8] {
    let ptr = data.as_ptr();
    // 故意把长度放大：能编译，运行却是未定义行为
    unsafe { std::slice::from_raw_parts(ptr, data.len() + 8) }
}

fn main() {}
