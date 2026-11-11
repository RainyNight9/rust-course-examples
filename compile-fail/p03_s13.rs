// 漏一个分支就编译不过
// 视频定位：P03 03:58
// 期望错误：E0004 —— 本文件故意编译失败，不要加入 workspace。
// error[E0004]: non-exhaustive patterns: `Ordering::Equal` not covered

use std::cmp::Ordering;

fn label(o: Ordering) -> &'static str {
    match o {
        Ordering::Less => "small",
        Ordering::Greater => "big",
    }
}
