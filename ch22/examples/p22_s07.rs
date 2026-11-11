// 包一层安全接口
// 视频定位：P22 01:59 · 运行：cargo run -p ch22 --example p22_s07
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub fn split_at_mut(slice: &mut [i32], mid: usize)
    -> (&mut [i32], &mut [i32])
{
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid),
         std::slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

fn main() {}
