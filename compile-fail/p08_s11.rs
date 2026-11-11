// 按值匹配会搬走数据
// 视频定位：P08 03:07
// 期望错误：E0382 —— 本文件故意编译失败，不要加入 workspace。
// error[E0382]: borrow of moved value: `name`

fn main() {
    let name: Option<String> = Some(String::from("alice"));
    if let Some(n) = name {
        println!("拿到 {n}");
    }
    println!("{:?}", name);   // name 已被移动
}
