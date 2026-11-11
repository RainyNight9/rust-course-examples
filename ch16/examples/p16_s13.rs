// 只要写 next 一个
// 视频定位：P16 03:46 · 运行：cargo run -p ch16 --example p16_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct Counter { count: u32 }
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 3 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {


    let sum: u32 = Counter { count: 0 }.sum();
    assert_eq!(sum, 6);
}
