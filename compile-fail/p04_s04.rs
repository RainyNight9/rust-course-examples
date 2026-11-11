// 想改必须先说一声
// 视频定位：P04 01:06
// 期望错误：E0384 —— 本文件故意编译失败，不要加入 workspace。
// error[E0384]: cannot assign twice to immutable variable `x`

fn main() {
    let x = 5;
    x = 6;
    println!("{x}");
}
