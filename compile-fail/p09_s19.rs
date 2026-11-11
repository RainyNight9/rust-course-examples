// mod 报 E0583
// 视频定位：P09 05:10
// 期望错误：E0583 —— 本文件故意编译失败，不要加入 workspace。
// error[E0583]: file not found for module `garden`

// src/main.rs —— 只写了声明，却忘了建对应文件
mod garden;

fn main() {
    garden::water();
}
