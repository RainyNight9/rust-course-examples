# NOTICE · 来源与署名

## 本仓库的内容来源

本仓库是 B 站《Rust 程序设计语言》22 集中文系统课的配套代码，代码与讲解要点依据下列资料改编整理：

| 来源 | 许可 | 链接 |
| --- | --- | --- |
| 《Rust 程序设计语言 简体中文版》（Rust 中文社区译） | MIT，Copyright (c) 2017-2025 Rust 中文社区 | <https://github.com/KaiserY/trpl-zh-cn> · <https://kaisery.github.io/trpl-zh-cn/> |
| The Rust Programming Language（rust-lang/book，上游原书） | Apache-2.0 或 MIT（任选其一） | <https://github.com/rust-lang/book> · <https://doc.rust-lang.org/book/> |

中文译本的 MIT 许可原文见 [LICENSE](LICENSE)，上游 MIT 许可原文见
[LICENSE-UPSTREAM-MIT](LICENSE-UPSTREAM-MIT)；上游 Apache-2.0 许可全文见
<https://github.com/rust-lang/book/blob/main/LICENSE-APACHE>。
以上许可均允许复制、修改、商用与再发布，本仓库按要求保留全部版权与许可声明。

## 改编说明

- 课程旁白为原创讲解口语稿；本仓库的代码片段来自课程逐镜脚本，脚本内容依据上述资料改写。
- 为让片段可以独立编译，部分示例补齐了 `main`、`use`，或把语句包进函数；
  P04 的整数溢出演示改用 `std::hint::black_box` 阻止编译期常量求值。
  所有未经改写的原始片段保留在 `docs/snippets/`，`tools/code_inventory.json` 保存了逐镜原始清单。
- `compile-fail/` 中的文件是课程中用于讲解编译器报错的反例，故意无法通过编译。
- 本仓库新增的整理性内容（目录结构、文档注释、索引、说明文档）同样以
  MIT / Apache-2.0 双许可提供，任选其一使用。

## 配套视频

22 集课程对应中文版全书 21 章；仓库内每个文件头部的 `Pxx mm:ss` 标记可用于在视频中定位。
