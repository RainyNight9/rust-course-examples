// 取走一个字段也算移动
// 视频定位：P05 05:00
// 期望错误：E0382 —— 本文件故意编译失败，不要加入 workspace。
// error[E0382]: borrow of partially moved value: `u`

#[derive(Debug)]
struct User { name: String, age: u8 }

fn main() {
    let u = User { name: String::from("A"), age: 18 };
    let n = u.name;          // name 被移动出去
    println!("{n}, {u:?}");  // u 整体已不可再用
}
