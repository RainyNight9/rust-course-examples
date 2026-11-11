// 返回局部变量必然失败
// 视频定位：P13 00:52
// 期望错误：E0515 —— 本文件故意编译失败，不要加入 workspace。
// error[E0515]: cannot return value referencing local variable `s`

fn bad() -> &str {
    let s = String::from("临时");
    &s
}
