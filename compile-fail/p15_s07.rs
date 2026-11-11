// 目标类型推不出
// 视频定位：P15 02:05
// 期望错误：E0282 —— 本文件故意编译失败，不要加入 workspace。
// error[E0282]: type annotations needed

use std::env;

let args = env::args().collect();
println!("{}", args.len());
