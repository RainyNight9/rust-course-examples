// 私有字段访问报错
// 视频定位：P21 01:09
// 期望错误：E0616 —— 本文件故意编译失败，不要加入 workspace。
// error[E0616]: field `balance` of struct `Account` is private

mod bank {
    pub struct Account { balance: i32 }
    impl Account {
        pub fn new() -> Self { Account { balance: 0 } }
        pub fn balance(&self) -> i32 { self.balance }
    }
}

fn main() {
    let a = bank::Account::new();
    println!("{}", a.balance);   // 直接读私有字段
}
