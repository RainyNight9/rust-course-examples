// 参数不写类型不行
// 视频定位：P04 04:37
// 期望错误：E0308 —— 本文件故意编译失败，不要加入 workspace。
// error[E0308]: mismatched types, expected `i32`, found `&str`

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let r = double("3");
    println!("{r}");
}
