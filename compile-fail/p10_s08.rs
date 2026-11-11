// 忘了标 mut
// 视频定位：P10 02:11
// 期望错误：E0596 —— 本文件故意编译失败，不要加入 workspace。
// error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable

fn main() {
    let v = vec![1, 2, 3];
    v.push(4);
    println!("{v:?}");
}
