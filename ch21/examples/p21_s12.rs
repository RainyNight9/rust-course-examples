// 把状态变成类型
// 视频定位：P21 03:19 · 运行：cargo run -p ch21 --example p21_s12
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub struct Post { content: String }
pub struct DraftPost { content: String }
pub struct PendingReviewPost { content: String }

impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}
impl Post {
    pub fn content(&self) -> &str { &self.content }
}

fn main() {}
