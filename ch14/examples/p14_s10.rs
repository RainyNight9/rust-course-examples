// 用问号省掉铺垫
// 视频定位：P14 03:06 · 运行：cargo run -p ch14 --example p14_s10
// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。
#![allow(dead_code, unused_variables, unused_mut, unused_imports,
         unused_must_use, unused_macros, unused_assignments,
         non_snake_case)]

#[test]
fn parse_config() -> Result<(), String> {
    let cfg = std::fs::read_to_string("config.toml")
        .map_err(|e| format!("读配置失败：{e}"))?;
    assert!(cfg.contains("port"), "配置里缺 port：{cfg:?}");
    Ok(())
}

fn main() {}
