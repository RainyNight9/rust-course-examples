// 转移的可能是副本
// 视频定位：P16 02:58
// 期望错误：E0382 —— 本文件故意编译失败，不要加入 workspace。
// error[E0382]: borrow of moved value: `s`

fn main() {
    let n = 5;
    let c = move || println!("{n}");
    c();
    println!("{n}");                 // i32 是 Copy，仍可用

    let s = String::from("hi");
    let c2 = move || println!("{s}");
    c2();
    println!("{s}");                 // 所有权已移走
}
