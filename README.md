# Rust 系统课 · 配套代码

B 站《Rust 程序设计语言》**22 集中文系统课**的配套代码仓库。课程覆盖中文版全书 21 章，
这里把视频里出现的代码整理成可以直接运行、对照阅读的工程。

- **119 个可运行示例**：位于 `chNN/examples/`，全部通过 `cargo build --workspace --examples`
- **59 个故意编译失败示例**：位于 `compile-fail/`，用来对照 E0382、E0499 等真实报错
- **84 段讲解摘录**：位于 `docs/snippets/`，是命令行、Cargo.toml、控制台输出或课程中的代码片段，
  本身不是完整程序，按原样保留

每个文件头部都标了**视频集数与时间点**，例如 `P05 02:41`，可以直接跳到视频对应位置。

## 配套资料

| 资料 | 位置 | 用途 |
| --- | --- | --- |
| [22 集学习手册（Markdown）](docs/学习手册.md) · [PDF 49 页](docs/学习手册.pdf) | `docs/` | 每集的目标、规则、易错点、随堂练习、知识地图与代码索引；PDF 可直接作为「三连领资料」发放 |
| [示例索引](docs/EXAMPLE_INDEX.md) | `docs/` | 119 个示例 × 视频时间点完整对照 |
| [6 篇精选图文长文](articles/README.md) | `articles/` | 所有权、借用与生命周期、泛型与 trait、错误处理、并发、异步；供专栏 / 掘金 / 知乎发布引流 |

## 快速开始

```bash
git clone <你的仓库地址>
cd rust-course-examples
cargo check --workspace --all-targets     # 校验全部示例，无需联网

# 运行某一集的某个示例
cargo run -p ch05 --example p05_s08

# 也可以直接编译单个文件（示例都不依赖第三方 crate）
rustc ch05/examples/p05_s08.rs -o /tmp/demo && /tmp/demo
```

`p05_s08` 这样的文件名含义是：**P05 第 8 张幻灯片**。完整对照表见 [示例索引](docs/EXAMPLE_INDEX.md)。

## 为什么分成三类

视频里的代码是给幻灯片排版用的，直接倒进仓库会产生大量编译不过的文件。所以做了分类：

| 类型 | 位置 | 处理方式 |
| --- | --- | --- |
| 可运行示例 | `chNN/examples/*.rs` | 补齐 `main`、`use` 等上下文后逐个用 `rustc` 验证，只有能编译的才放进来 |
| 故意报错 | `compile-fail/*.rs` | 课程里用来讲错误码的反例，保留原样并标注期望的 `E0xxx`，**不加入 workspace** |
| 讲解摘录 | `docs/snippets/pNN.md` | 命令行、配置、输出、依赖第三方 crate（rand / tokio / futures）的片段，以及无法独立成程序的语句摘录 |

所有示例**只用标准库**，因此 `cargo check` 完全离线可跑。涉及 `rand`、`tokio`、`futures` 的课程片段
（如猜数字游戏、异步章节）保留在摘录里，并注明需要添加的依赖。

> 为便于编译，部分示例做了等价补齐（补 `main`、补 `use`、把语句包进函数）；
> P04 的整数溢出演示改用 `std::hint::black_box` 阻止编译期常量求值，以便观察 debug / release 差异。
> 原始课程代码始终保留在 `docs/snippets/` 中可对照。

## 22 集索引

| 集 | 标题 | 对应章节 | 视频时长(分) | 可运行 | 报错示例 | 摘录 | 目录 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| P01 | 这本书该怎么读 | 全书导览 | 10.44 | 2 | 2 | 1 | [ch01](ch01/) · [摘录](docs/snippets/p01.md) |
| P02 | 装好环境，跑起第一行代码 | 第 1 章 入门指南 | 8.24 | 3 | 1 | 8 | [ch02](ch02/) · [摘录](docs/snippets/p02.md) |
| P03 | 一个小游戏串起基础语法 | 第 2 章 猜数字游戏 | 8.81 | 2 | 2 | 8 | [ch03](ch03/) · [摘录](docs/snippets/p03.md) |
| P04 | 变量、类型、函数、控制流 | 第 3 章 常见编程概念 | 8.55 | 7 | 3 | 3 | [ch04](ch04/) · [摘录](docs/snippets/p04.md) |
| P05 | 所有权与移动 | 第 4 章 所有权（上） | 8.85 | 6 | 3 | 1 | [ch05](ch05/) · [摘录](docs/snippets/p05.md) |
| P06 | 引用、借用与 slice | 第 4 章 所有权（下） | 8.87 | 7 | 5 | 0 | [ch06](ch06/) · [摘录](docs/snippets/p06.md) |
| P07 | 用结构体组织数据 | 第 5 章 结构体 | 8.34 | 3 | 6 | 2 | [ch07](ch07/) · [摘录](docs/snippets/p07.md) |
| P08 | 表达「几种可能之一」 | 第 6 章 枚举与模式匹配 | 8.16 | 4 | 3 | 4 | [ch08](ch08/) · [摘录](docs/snippets/p08.md) |
| P09 | 包、Crate 与模块 | 第 7 章 模块系统 | 8.14 | 1 | 4 | 6 | [ch09](ch09/) · [摘录](docs/snippets/p09.md) |
| P10 | Vec、String 与 HashMap | 第 8 章 常见集合 | 8.18 | 8 | 4 | 1 | [ch10](ch10/) · [摘录](docs/snippets/p10.md) |
| P11 | panic! 与 Result | 第 9 章 错误处理 | 8.26 | 6 | 4 | 3 | [ch11](ch11/) · [摘录](docs/snippets/p11.md) |
| P12 | 泛型与 trait | 第 10 章（上） | 7.93 | 7 | 3 | 3 | [ch12](ch12/) · [摘录](docs/snippets/p12.md) |
| P13 | 生命周期 | 第 10 章（下） | 8.87 | 6 | 5 | 2 | [ch13](ch13/) · [摘录](docs/snippets/p13.md) |
| P14 | 把测试写进项目里 | 第 11 章 自动化测试 | 9.00 | 6 | 1 | 4 | [ch14](ch14/) · [摘录](docs/snippets/p14.md) |
| P15 | 做一个 minigrep | 第 12 章 命令行项目 | 8.80 | 5 | 2 | 7 | [ch15](ch15/) · [摘录](docs/snippets/p15.md) |
| P16 | 函数式特性 | 第 13 章 迭代器与闭包 | 8.60 | 9 | 3 | 2 | [ch16](ch16/) · [摘录](docs/snippets/p16.md) |
| P17 | 发布、文档与工作空间 | 第 14 章 Cargo 进阶 | 8.59 | 3 | 0 | 11 | [ch17](ch17/) · [摘录](docs/snippets/p17.md) |
| P18 | Box、Rc 与 RefCell | 第 15 章 智能指针 | 8.60 | 12 | 1 | 1 | [ch18](ch18/) · [摘录](docs/snippets/p18.md) |
| P19 | 线程、消息与共享状态 | 第 16 章 无畏并发 | 8.30 | 8 | 1 | 2 | [ch19](ch19/) · [摘录](docs/snippets/p19.md) |
| P20 | async、await 与 Future | 第 17 章 异步编程 | 8.15 | 2 | 2 | 9 | [ch20](ch20/) · [摘录](docs/snippets/p20.md) |
| P21 | 面向对象与模式语法 | 第 18–19 章 | 8.23 | 6 | 3 | 2 | [ch21](ch21/) · [摘录](docs/snippets/p21.md) |
| P22 | 高级特性与最终项目 | 第 20–21 章 | 8.33 | 6 | 1 | 4 | [ch22](ch22/) · [摘录](docs/snippets/p22.md) |

## 目录结构

```
rust-course-examples/
├── Cargo.toml              # workspace，22 个章节 crate
├── ch01 … ch22/            # 每集一个 crate，examples/ 下是该文件对应的可运行示例
├── compile-fail/           # 故意编译失败的反例 + README（含错误码索引）
├── docs/
│   ├── 学习手册.md / .html / .pdf   # 22 集学习手册（PDF 49 页）
│   ├── EXAMPLE_INDEX.md    # 全部可运行示例 × 视频时间点对照表
│   └── snippets/           # 按集整理的命令行 / 配置 / 教学摘录
├── articles/               # 6 篇精选图文长文 + 发布索引
├── tools/                  # 从课程逐镜数据重建本仓库的脚本与原始清单
├── LICENSE                 # 中文译本 MIT 许可原文
├── LICENSE-UPSTREAM-MIT    # rust-lang/book MIT 许可原文
└── NOTICE.md               # 来源、改编说明与署名
```

## 校验

```bash
cargo check --workspace --all-targets
cargo build --workspace --examples
```

注意：**不要**直接跑 `cargo test --workspace --all-targets`。部分示例本身就是用来演示
`panic!`、断言失败或测试失败的，整体跑会被预期内的失败打断；请按需逐个 `cargo run` 观察。

## 许可与署名

本仓库依据《Rust 程序设计语言 简体中文版》（MIT，Copyright (c) 2017-2025 Rust 中文社区）
与上游 `rust-lang/book`（Apache-2.0 / MIT 双许可）改编整理。两者均允许商用与演绎，
本仓库保留其版权与许可声明，详见 [NOTICE.md](NOTICE.md)。

- 中文版在线阅读：<https://kaisery.github.io/trpl-zh-cn/>
- 官方文档：<https://doc.rust-lang.org/book/>
