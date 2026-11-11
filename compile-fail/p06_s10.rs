// 三处必须对齐
// 视频定位：P06 03:01
// 期望错误：E0596 —— 本文件故意编译失败，不要加入 workspace。
// error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable

fn push(s: &mut String) { s.push_str("!"); }

fn main() {
    let s = String::from("hi");   // 忘了 mut
    push(&mut s);
}
