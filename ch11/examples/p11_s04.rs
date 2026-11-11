// 把崩溃写成测试
// 视频定位：P11 01:15 · 运行：cargo run -p ch11 --example p11_s04
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为零");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::divide;

    #[test]
    #[should_panic(expected = "除数不能为零")]
    fn rejects_zero() {
        divide(10, 0);
    }
}

fn main() {}
