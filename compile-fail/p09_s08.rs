// 私有模块报 E0603
// 视频定位：P09 02:07
// 期望错误：E0603 —— 本文件故意编译失败，不要加入 workspace。
// error[E0603]: module `inner` is private

mod outer {
    mod inner {
        pub fn secret() {}
    }
}

fn main() {
    crate::outer::inner::secret();
}
