// 缺 Display
// 视频定位：P12 04:58
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: the trait bound `Config: Display` is not satisfied

use std::fmt::Display;

fn notify<T: Display>(item: T) {
    println!("公告：{item}");
}

struct Config { port: u16 }

fn main() {
    notify(Config { port: 8080 });
}
