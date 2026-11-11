// 离开作用域时做点事
// 视频定位：P18 03:34 · 运行：cargo run -p ch18 --example p18_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct CustomSmartPointer { data: String }
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("正在清理 {}", self.data);
    }
}

fn main() {


    let c = CustomSmartPointer { data: String::from("资源") };
    drop(c);    // 提前释放
}
