// let 触发 E0005
// 视频定位：P21 04:39
// 期望错误：E0005 —— 本文件故意编译失败，不要加入 workspace。
// error[E0005]: refutable pattern in local binding: `None` not covered

fn main() {
    let opt: Option<i32> = Some(7);
    let Some(x) = opt;   // 可反驳模式放进 let
    println!("{x}");
}
