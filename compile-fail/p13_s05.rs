// 编译器看不出哪个还活着
// 视频定位：P13 01:32
// 期望错误：E0106 —— 本文件故意编译失败，不要加入 workspace。
// error[E0106]: missing lifetime specifier

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
