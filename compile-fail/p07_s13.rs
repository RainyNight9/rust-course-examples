// 打印结构体要Debug
// 视频定位：P07 03:37
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: `Rectangle` doesn't implement `Debug`

struct Rectangle { width: u32, height: u32 }

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect = {:?}", rect);
}
