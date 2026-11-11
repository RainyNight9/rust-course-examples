// 分支类型要统一
// 视频定位：P08 04:13
// 期望错误：E0308 —— 本文件故意编译失败，不要加入 workspace。
// error[E0308]: `match` arms have incompatible types

enum Coin { Penny, Nickel, Dime, Quarter }

fn main() {
    let coin = Coin::Quarter;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => String::from("25"),
    };
    println!("{value}");
}
