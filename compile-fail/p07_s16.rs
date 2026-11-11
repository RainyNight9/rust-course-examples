// 可变方法要声明
// 视频定位：P07 04:21
// 期望错误：E0596 —— 本文件故意编译失败，不要加入 workspace。
// error[E0596]: cannot borrow `rect` as mutable, as it is not declared as mutable

struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn widen(&mut self, n: u32) {
        self.width += n;
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    rect.widen(10);
}
