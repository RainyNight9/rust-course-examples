// ! 能转成任意类型
// 视频定位：P22 03:03 · 运行：cargo run -p ch22 --example p22_s11
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn fail(msg: &str) -> ! {
    panic!("{msg}")
}

fn choose(flag: bool) -> i32 {
    if flag { 1 } else { fail("走不到") }
}

fn main() {
    let n: u32 = match Some(3) {
        Some(v) => v,
        None => return,   // return / break / continue 的类型也是 !
    };
    println!("{n} {}", choose(true));
}
