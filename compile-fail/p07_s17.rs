// 谁会拿走实例
// 视频定位：P07 04:40
// 期望错误：E0382 —— 本文件故意编译失败，不要加入 workspace。
// error[E0382]: borrow of moved value: `rect`

struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn into_width(self) -> u32 { self.width }
    fn area(&self) -> u32 { self.width * self.height }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("{}", rect.into_width());
    println!("{}", rect.area());   // rect 已被消耗
}
