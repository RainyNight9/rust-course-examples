// @ 绑定 + 匹配守卫
// 视频定位：P21 05:27 · 运行：cargo run -p ch21 --example p21_s20
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn classify(p: Point) {
    match p {
        Point { x, .. } if x < 0 => println!("左半平面：{x}"),
        Point { x: a @ 0..=10, y } => println!("x={a}, y={y}"),
        other => println!("其它 {other:?}"),
    }
}

fn main() { classify(Point { x: 7, y: 3 }); }
