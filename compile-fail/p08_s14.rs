// 漏一个分支就报错
// 视频定位：P08 03:52
// 期望错误：E0004 —— 本文件故意编译失败，不要加入 workspace。
// error[E0004]: non-exhaustive patterns: `Direction::Right` not covered

enum Direction { Up, Down, Left, Right }

fn main() {
    let d = Direction::Up;
    match d {
        Direction::Up => println!("上"),
        Direction::Down => println!("下"),
        Direction::Left => println!("左"),
    }
}
