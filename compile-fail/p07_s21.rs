// 点号调不了关联函数
// 视频定位：P07 05:46
// 期望错误：E0599 —— 本文件故意编译失败，不要加入 workspace。
// error[E0599]: no method named `square` found for struct `Rectangle` in the current scope

struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let sq = rect.square(10);   // 关联函数不能用点号
}
