// struct 的基本写法
// 视频定位：P07 01:44 · 运行：cargo run -p ch07 --example p07_s06
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("alice"),
        sign_in_count: 1,
    };
    user1.sign_in_count += 1;
}
