// 别忘了 Err 分支
// 视频定位：P11 02:52
// 期望错误：E0004 —— 本文件故意编译失败，不要加入 workspace。
// error[E0004]: non-exhaustive patterns: `Err(_)` not covered

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => println!("打开成功：{file:?}"),
    }
}
