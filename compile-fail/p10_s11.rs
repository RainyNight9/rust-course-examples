// 引用没用完别 push
// 视频定位：P10 02:54
// 期望错误：E0502 —— 本文件故意编译失败，不要加入 workspace。
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("第一个是 {first}");
}
