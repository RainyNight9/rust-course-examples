// 第二次调用就锁死
// 视频定位：P16 00:51
// 期望错误：E0308 —— 本文件故意编译失败，不要加入 workspace。
// error[E0308]: mismatched types

let example = |x| x;
let s = example(String::from("hi"));
let n = example(5);
