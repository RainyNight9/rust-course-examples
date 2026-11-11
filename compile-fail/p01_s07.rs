// 一个报错，胜过一次线上事故
// 视频定位：P01 02:28
// 期望错误：E0382 —— 本文件故意编译失败，不要加入 workspace。
// error[E0382]: borrow of moved value: `s1`

let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!");
