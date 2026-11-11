// 去掉就立刻报错
// 视频定位：P15 04:45
// 期望错误：E0106 —— 本文件故意编译失败，不要加入 workspace。
// error[E0106]: missing lifetime specifier

pub fn search(query: &str, contents: &str) -> Vec<&str> {
    contents.lines()
        .filter(|l| l.contains(query))
        .collect()
}
