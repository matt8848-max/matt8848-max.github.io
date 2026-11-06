#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""在脚本所在目录下递归查找所有 .md 文件，
统计包含关键词 "#AI" 的文件并输出命中次数与相对路径。"""

import sys
from collections.abc import Iterator
from pathlib import Path
from typing import Final

KEYWORD: Final[str] = "#AI"
SCRIPT_DIR: Final[Path] = Path(__file__).resolve().parent

# 任务清单本身用「#AI任务：」之类的行来分配任务，必然命中关键词，
# 但它不是待编写的正文，故从扫描结果中排除，避免每次都出现假阳性。
EXCLUDED_FILE_NAMES: Final[frozenset[str]] = frozenset({"任务清单.md"})


def iter_markdown(root: Path) -> Iterator[Path]:
    """递归遍历 root，跳过隐藏目录（以 . 开头）与排除名单里的文件，产出所有 .md 文件。"""
    for entry in root.iterdir():
        if entry.name.startswith("."):
            continue
        if entry.is_dir() and not entry.is_symlink():
            yield from iter_markdown(entry)
        elif (
            entry.is_file()
            and entry.suffix.lower() == ".md"
            and entry.name not in EXCLUDED_FILE_NAMES
        ):
            yield entry


def main() -> None:
    results: list[tuple[str, int]] = []

    for path in iter_markdown(SCRIPT_DIR):
        try:
            content: str = path.read_text(encoding="utf-8", errors="ignore")
        except OSError as exc:
            print(f"读取失败: {path}: {exc}", file=sys.stderr)
            continue

        count: int = content.count(KEYWORD)
        if count > 0:
            rel: str = path.relative_to(SCRIPT_DIR).as_posix()
            results.append((rel, count))

    if not results:
        print(f'未找到包含 "{KEYWORD}" 的 md 文件。')
        return

    results.sort(key=lambda item: (-item[1], item[0]))
    total: int = sum(count for _, count in results)

    print(f'关键词 "{KEYWORD}"：{len(results)} 个文件，共 {total} 次\n')
    width: int = max(len(rel) for rel, _ in results)
    for rel, count in results:
        print(f"{rel:<{width}}  {count:>4}")


if __name__ == "__main__":
    main()