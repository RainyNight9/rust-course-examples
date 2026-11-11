// void 里用不了 ?
// 视频定位：P11 04:56
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`

use std::fs::File;

fn main() {
    let _f = File::open("hello.txt")?;
}
