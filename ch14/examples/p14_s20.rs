// 正常错误都测到
// 视频定位：P14 06:15 · 运行：cargo run -p ch14 --example p14_s20
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err(String::from("除数不能为 0")) } else { Ok(a / b) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ok() { assert_eq!(divide(10, 2), Ok(5)); }
    #[test]
    fn zero() { assert!(divide(1, 0).is_err()); }
    #[test]
    #[should_panic(expected = "除数不能为 0")]
    fn unwrap_zero() { divide(1, 0).unwrap(); }
}

fn main() {}
