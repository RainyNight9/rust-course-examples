// 局部引用变不成静态
// 视频定位：P13 06:19
// 期望错误：E0597 —— 本文件故意编译失败，不要加入 workspace。
// error[E0597]: `s` does not live long enough

fn leak_it() -> &'static str {
    let s = String::from("局部");
    &s
}
