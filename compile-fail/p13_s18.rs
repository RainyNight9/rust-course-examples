// impl 要先声明标记
// 视频定位：P13 05:22
// 期望错误：E0261 —— 本文件故意编译失败，不要加入 workspace。
// error[E0261]: use of undeclared lifetime name `'a`

struct Excerpt<'a> { part: &'a str }

impl Excerpt<'a> {
    fn part(&self) -> &str { self.part }
}
