// 不装箱算不出大小
// 视频定位：P18 01:50
// 期望错误：E0072 —— 本文件故意编译失败，不要加入 workspace。
// error[E0072]: recursive type `List` has infinite size

enum List {
    Cons(i32, List),
    Nil,
}
