// 自定义类型缺了什么
// 视频定位：P14 01:48
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: `Secret` doesn't implement `std::fmt::Debug`

struct Secret(i32);

#[test]
fn compare() {
    let a = Secret(1);
    let b = Secret(2);
    assert_eq!(a, b);
}
