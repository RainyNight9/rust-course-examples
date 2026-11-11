// 局部变量逆序释放
// 视频定位：P18 02:59 · 运行：cargo run -p ch18 --example p18_s10
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct Tag(&'static str);
impl Drop for Tag {
    fn drop(&mut self) { println!("释放 {}", self.0); }
}

fn main() {
    let _a = Tag("a");
    let _b = Tag("b");
}
