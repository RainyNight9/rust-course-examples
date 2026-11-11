// 返回的引用没来源
// 视频定位：P06 05:27
// 期望错误：E0106 —— 本文件故意编译失败，不要加入 workspace。
// error[E0106]: missing lifetime specifier

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
