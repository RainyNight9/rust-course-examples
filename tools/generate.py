#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""从 Rust 22 集课程数据生成配套代码仓库。

输入: /tmp/rust_code_inventory.json (262 个代码镜头)
      course/subs/epNN/segments.json (逐镜时间点)
      course/publish.csv (集标题 / 章节)
输出: rust-course-examples/ 仓库
"""
import csv
import json
import re
import shutil
import subprocess
from pathlib import Path

COURSE = Path('/Users/rain9/WorkBuddy/2026-11-09-15-16-30/rust-video/course')
REPO = Path(__file__).resolve().parent.parent
INV = json.loads((Path(__file__).resolve().parent / 'code_inventory.json').read_text(encoding='utf-8'))
CHECK_DIR = Path('/tmp/rustcheck')
try:
    CHECK_DIR.mkdir(parents=True, exist_ok=True)
except FileExistsError:
    pass

PUB = {}
with open(COURSE / 'publish.csv', encoding='utf-8-sig') as f:
    for row in list(csv.reader(f))[1:]:
        PUB[int(row[0][1:])] = dict(title=row[1], chap=row[2], minutes=row[3])

SEGS = {}
for ep in range(1, 23):
    p = COURSE / 'subs' / f'ep{ep:02d}' / 'segments.json'
    if p.exists():
        SEGS[ep] = json.loads(p.read_text(encoding='utf-8'))


def stamp(ep, slide):
    segs = SEGS.get(ep) or []
    for s in segs:
        if s['n'] == slide:
            t = int(s['start'])
            return f'P{ep:02d} {t//60:02d}:{t%60:02d}'
    return f'P{ep:02d}'


def clean_title(t):
    return re.sub(r'<[^>]+>', '', t).replace('&amp;', '&').strip()


def error_code(err):
    m = re.search(r'E\d{4}', err or '')
    return m.group(0) if m else '—'


def classify(src):
    s = src.strip()
    lines = [l for l in s.splitlines() if l.strip()]
    first = lines[0].strip() if lines else ''
    dollars = sum(1 for l in lines if l.strip().startswith('$'))
    if dollars / max(len(lines), 1) > 0.3:
        return 'cmd'
    if re.match(r'^(\[package\]|\[dependencies\]|\[profile|\[workspace\]|# Cargo\.toml|// Cargo\.toml|// src/|running \d+ test|test tests::|\s+Compiling)', first):
        return 'config'
    if re.search(r'\b(rand::|tokio|futures::|serde::|reqwest::|chrono::)', s):
        return 'dep'
    return 'rust'


TOPLEVEL = re.compile(r'^(fn |async fn |pub fn |pub async fn |struct |pub struct |enum |pub enum |impl |trait |pub trait |mod |pub mod |use |pub use |#\[|const |static |type |macro_rules!|extern crate|///|//!|pub type )')


HEADER = ('// 本文件由课程逐镜脚本自动抽取整理，用于配合视频阅读。\n'
          '#![allow(dead_code, unused_variables, unused_mut, unused_imports,\n'
          '         unused_must_use, unused_macros, unused_assignments,\n'
          '         non_snake_case)]\n\n')


def indent(lines):
    return '\n'.join('    ' + l if l.strip() else '' for l in lines)


def candidates(src):
    """一个教学片段可能有多种补法，逐个交给 rustc 试，取第一个能编译的。"""
    s = src.rstrip()
    lines = s.splitlines()
    out = []
    if 'fn main' in s:
        out.append(('原样', HEADER + s + '\n'))
    else:
        out.append(('补 main', HEADER + s + '\n\nfn main() {}\n'))

    # 智能拆分：顶层定义（含其块内行）留在外面，其余语句包进 main
    outer, inner, depth, in_item = [], [], 0, False
    for l in lines:
        st = l.strip()
        if depth == 0 and not in_item:
            if TOPLEVEL.match(st):
                in_item = True
                outer.append(l)
                depth += l.count('{') - l.count('}')
                if depth <= 0 and (st.endswith(';') or st.endswith('}')):
                    in_item, depth = False, 0
                continue
            inner.append(l)
        else:
            outer.append(l)
            depth += l.count('{') - l.count('}')
            if depth <= 0:
                in_item, depth = False, 0
    if inner:
        body = '\n'.join(outer) + '\n\nfn main() {\n' + indent(inner) + '\n}\n'
        out.append(('拆分补 main', HEADER + body))

    # 兜底：整段包进 main（use / 嵌套定义在函数内也合法）
    body = 'fn main() {\n' + indent(lines) + '\n}\n'
    out.append(('包进 main', HEADER + body))
    return out


def rustc_ok(code, base):
    path = CHECK_DIR / f'{base}.rs'
    path.write_text(code, encoding='utf-8')
    r = subprocess.run(
        ['rustc', '--edition', '2024', '--crate-type', 'lib', '--test',
         '--emit', 'metadata', '--out-dir', str(CHECK_DIR), str(path)],
        capture_output=True, text=True, timeout=60)
    if r.returncode == 0:
        return True, ''
    errs = [l for l in r.stderr.splitlines() if 'error' in l.lower()]
    return False, (errs[0][:160] if errs else r.stderr.strip()[:160])


# ---- 清理并建骨架（只重建代码目录，绝不删 README / LICENSE / docs / tools） ----
REPO.mkdir(parents=True, exist_ok=True)
for d in list(REPO.glob('ch*')):
    if d.is_dir():
        shutil.rmtree(d)
if (REPO / 'compile-fail').exists():
    shutil.rmtree(REPO / 'compile-fail')
(REPO / 'compile-fail').mkdir()
for p in (REPO / 'docs' / 'snippets', REPO / 'tools'):
    p.mkdir(parents=True, exist_ok=True)

(REPO / 'Cargo.toml').write_text(
    '[workspace]\nresolver = "2"\nmembers = [\n'
    + ''.join(f'    "ch{ep:02d}",\n' for ep in range(1, 23))
    + ']\n', encoding='utf-8')

for ep in range(1, 23):
    d = REPO / f'ch{ep:02d}'
    (d / 'examples').mkdir(parents=True)
    (d / 'src').mkdir()
    (d / 'Cargo.toml').write_text(
        f'[package]\nname = "ch{ep:02d}"\nversion = "0.1.0"\nedition = "2024"\n'
        'publish = false\n\n[dependencies]\n', encoding='utf-8')
    (d / 'src' / 'lib.rs').write_text(
        f'//! P{ep:02d} · {PUB[ep]["title"]}（{PUB[ep]["chap"]}）\n'
        '//! 本 crate 不含库代码，可运行示例见 examples/ 目录。\n', encoding='utf-8')

entries = []
for x in INV:
    ep, slide = x['ep'], x['slide']
    title = clean_title(x['title'])
    src, cap, err = x['src'], re.sub(r'<[^>]+>', '', x['cap'] or '').strip(), x['err']
    # P04#11 演示整数溢出：原样会被编译期常量求值拒绝，等价改写为运行时求值
    src = src.replace('let a: u8 = 255;',
                      '// 用 black_box 阻止编译期常量求值：debug 下运行时 panic，release 下回绕到 0\n'
                      '    let a: u8 = std::hint::black_box(255);')
    base = f'p{ep:02d}_s{slide:02d}'
    rec = dict(ep=ep, slide=slide, title=title, base=base, src=src,
               cap=cap, err=err, stamp=stamp(ep, slide),
               kind=None, style='', reason='', code=error_code(err))

    if err:
        rec['kind'] = 'compile-fail'
        (REPO / 'compile-fail' / f'{base}.rs').write_text(
            f'// {title}\n// 视频定位：{rec["stamp"]}\n'
            f'// 期望错误：{rec["code"]} —— 本文件故意编译失败，不要加入 workspace。\n'
            f'// {err[:200].replace(chr(10), " ")}\n\n{src.rstrip()}\n', encoding='utf-8')
        entries.append(rec)
        continue

    cat = classify(src)
    if cat != 'rust':
        rec['kind'] = 'snippet'
        rec['reason'] = {'cmd': '命令行示例', 'config': '配置 / 输出片段', 'dep': '需第三方依赖'}[cat]
        entries.append(rec)
        continue

    chosen = None
    for style, code in candidates(src):
        ok, why = rustc_ok(code, base)
        if ok:
            chosen = (style, code)
            break
    if chosen:
        style, code = chosen
        rec['kind'] = 'example'
        rec['style'] = style
        (REPO / f'ch{ep:02d}' / 'examples' / f'{base}.rs').write_text(
            f'// {title}\n// 视频定位：{rec["stamp"]} · 运行：cargo run -p ch{ep:02d} --example {base}\n'
            + code, encoding='utf-8')
    else:
        rec['kind'] = 'snippet'
        rec['reason'] = '教学摘录，非完整程序'
        (CHECK_DIR / f'{base}.rs').unlink(missing_ok=True)
    entries.append(rec)

Path('/tmp/rust_repo_entries.json').write_text(
    json.dumps(entries, ensure_ascii=False, indent=1), encoding='utf-8')
(REPO / 'tools' / 'entries.json').write_text(
    json.dumps(entries, ensure_ascii=False, indent=1), encoding='utf-8')

from collections import Counter
print('kinds:', dict(Counter(e['kind'] for e in entries)))
for ep in range(1, 23):
    c = Counter(e['kind'] for e in entries if e['ep'] == ep)
    print(f'P{ep:02d} examples={c["example"]:2d} compile-fail={c["compile-fail"]:2d} snippets={c["snippet"]:2d}')
