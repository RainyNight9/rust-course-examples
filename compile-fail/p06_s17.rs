// 返回值里没有可借的东西
// 视频定位：P06 05:16
// 期望错误：— —— 本文件故意编译失败，不要加入 workspace。
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
