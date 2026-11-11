// 两个可变引用撞一起
// 视频定位：P06 04:01
// 期望错误：E0499 —— 本文件故意编译失败，不要加入 workspace。
// error[E0499]: cannot borrow `v` as mutable more than once at a time

fn main() {
    let mut v = vec![1, 2, 3];
    let a = &mut v[0];
    let b = &mut v[1];
    println!("{a} {b}");
}
