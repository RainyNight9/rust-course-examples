// 加分号，值就没了
// 视频定位：P04 05:15
// 期望错误：E0308 —— 本文件故意编译失败，不要加入 workspace。
// error[E0308]: mismatched types, expected `i32`, found `()`

fn add_one(x: i32) -> i32 {
    x + 1;
}
