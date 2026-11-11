// 目标类型省不掉
// 视频定位：P16 04:59
// 期望错误：E0282 —— 本文件故意编译失败，不要加入 workspace。
// error[E0282]: type annotations needed

let v = vec![1, 2, 3];
let out = v.iter().map(|x| x * 2).collect();
