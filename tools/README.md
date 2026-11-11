# 生成工具

- `code_inventory.json`：22 集课程里全部 262 个代码镜头的原始清单（集数、镜号、标题、代码、说明、错误标注）。
- `entries.json`：`generate.py` 的分类结果，含每个片段的归类、视频时间点与错误码，`docs/` 下的索引与摘录文档由它生成。
- `generate.py`：重建 `chNN/` 与 `compile-fail/`。逐个调用 `rustc` 验证片段能否编译，能编译的进 `examples/`，其余留待归入 `docs/snippets/`。

脚本只重建上述两个代码目录，**不会删除 README、LICENSE、NOTICE、docs 或 tools**。视频时间点取自课程项目的 `course/subs/epNN/segments.json`（缺失时退化为只标集数）。

**注意**：重跑 `generate.py` 会覆盖 `chNN/` 里的手工改动，如有修改请先同步进脚本。
