// 字段为什么报错
// 视频定位：P07 01:57
// 期望错误：E0106 —— 本文件故意编译失败，不要加入 workspace。
// error[E0106]: missing lifetime specifier

struct User {
    username: &str,
    sign_in_count: u64,
}

fn main() {
    let name = String::from("alice");
    let u = User { username: &name, sign_in_count: 1 };
    println!("{}", u.username);
}
