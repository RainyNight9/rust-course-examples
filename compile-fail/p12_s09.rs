// 部分方法看类型
// 视频定位：P12 02:22
// 期望错误：E0599 —— 本文件故意编译失败，不要加入 workspace。
// error[E0599]: no method named `x` found for struct `Point<i32, f64>` in the current scope

struct Point<T, U> { x: T, y: U }

impl<T> Point<T, T> {
    fn x(&self) -> &T { &self.x }
}

fn main() {
    let p = Point { x: 5, y: 2.5 };
    println!("{}", p.x());
}
