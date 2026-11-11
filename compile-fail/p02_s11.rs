// 少写 main 会报 E0601
// 视频定位：P02 03:05
// 期望错误：E0601 —— 本文件故意编译失败，不要加入 workspace。
// error[E0601]: main function not found in crate

fn greet() {
    println!("hi");
}
