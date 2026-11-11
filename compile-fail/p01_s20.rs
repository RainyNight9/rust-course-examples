// 同一份数据，两次可变借用
// 视频定位：P01 07:10
// 期望错误：E0499 —— 本文件故意编译失败，不要加入 workspace。
// error[E0499]: cannot borrow `v` as mutable more than once

let mut v = vec![1, 2, 3];
let first = &mut v[0];
let second = &mut v[1];

println!("{first} {second}");
