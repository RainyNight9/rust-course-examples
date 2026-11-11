// 不写 : u32 就推不出来
// 视频定位：P03 04:54
// 期望错误：E0282 —— 本文件故意编译失败，不要加入 workspace。
// error[E0282]: type annotations needed for the result of `parse`

fn main() {
    let text = "42";
    let n = text.parse().unwrap();
    println!("{n}");
}
