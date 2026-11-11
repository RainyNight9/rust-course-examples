// 两个 &str 不能加
// 视频定位：P10 04:53
// 期望错误：E0369 —— 本文件故意编译失败，不要加入 workspace。
// error[E0369]: cannot add `&str` to `&str`

fn main() {
    let greeting = "Hello, " + "world!";
    println!("{greeting}");
}
