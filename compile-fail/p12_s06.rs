// 缺 Copy 约束
// 视频定位：P12 01:39
// 期望错误：E0277 —— 本文件故意编译失败，不要加入 workspace。
// error[E0277]: the trait bound `String: Copy` is not satisfied

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let words = vec![String::from("a"), String::from("b")];
    println!("{}", largest(&words));
}
