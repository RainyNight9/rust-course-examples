// collect 要标注
// 视频定位：P10 03:46
// 期望错误：E0282 —— 本文件故意编译失败，不要加入 workspace。
// error[E0282]: type annotations needed

fn main() {
    let doubled = vec![1, 2, 3]
        .into_iter()
        .map(|x| x * 2)
        .collect();
    println!("{doubled:?}");
}
