// 末尾别忘包 Ok
// 视频定位：P11 04:27
// 期望错误：E0308 —— 本文件故意编译失败，不要加入 workspace。
// error[E0308]: mismatched types

use std::io::{self, Read};
use std::fs::File;

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    s                       // 忘了包 Ok
}
