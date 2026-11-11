// 守护让 future 不 Send
// 视频定位：P20 05:22
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: future cannot be sent between threads safely

use std::sync::Mutex;

#[tokio::main]
async fn main() {
    let m = Mutex::new(0);
    tokio::spawn(async move {
        let guard = m.lock().unwrap();
        tokio::task::yield_now().await;
        println!("{}", *guard);
    });
}
