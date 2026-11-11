// 私有构造 + PhantomData
// 视频定位：P21 03:33 · 运行：cargo run -p ch21 --example p21_s13
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

use std::marker::PhantomData;

struct Draft;
struct Published;
struct Post<State> {
    body: String,
    state: PhantomData<State>,
}

impl Post<Draft> {
    fn new(body: &str) -> Self {
        Post { body: body.into(), state: PhantomData }
    }
    fn publish(self) -> Post<Published> {
        Post { body: self.body, state: PhantomData }
    }
}
impl Post<Published> {
    fn url(&self) -> String { format!("/posts/{}", self.body.len()) }
}

fn main() {}
