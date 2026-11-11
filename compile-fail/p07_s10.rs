// 更新语法的部分移动
// 视频定位：P07 02:49
// 期望错误：E0382 —— 本文件故意编译失败，不要加入 workspace。
// error[E0382]: borrow of moved value: `user1.username`

struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("alice"),
        sign_in_count: 1,
    };
    let user2 = User {
        username: String::from("bob"),
        ..user1
    };
    println!("{}", user1.active);       // bool 是 Copy，还能用
    println!("{}", user1.username);     // 已被移动
}
