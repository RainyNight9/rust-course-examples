// 较短那方说了算
// 视频定位：P13 02:07
// 期望错误：E0597 —— 本文件故意编译失败，不要加入 workspace。
// error[E0597]: `inner` does not live long enough

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let outer = String::from("长长长");
    let result;
    {
        let inner = String::from("短");
        result = longest(&outer, &inner);
    }
    println!("{result}");
}
