// E 得Debug才行
// 视频定位：P11 03:40
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: `MyError` doesn't implement `Debug`

struct MyError;

fn main() {
    let r: Result<i32, MyError> = Err(MyError);
    println!("{}", r.unwrap());
}
