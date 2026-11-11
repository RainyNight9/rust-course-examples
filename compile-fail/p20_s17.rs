// future 往往是 !Unpin
// 视频定位：P20 04:35
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: the trait bound `{async block@...}: Unpin` is not satisfied

let fut = async {
    println!("hi");
    1
};
// let p = Pin::new(&mut fut); // 编译不过：future 是 !Unpin
let p = Box::pin(fut);         // 堆上地址稳定，才能安全 poll
let _ = p;
