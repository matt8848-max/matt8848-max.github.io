#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""把「文本可读」的 PDF 转成 mdbook 章节草稿（正文 + 表格）。

为什么这样分工（详见 .agents/skills/mdbookwrite/SKILL.md「文本可读 PDF 的转写」）：
+ 正文用 pymupdf：按版面顺序输出，中西文混排（`X 射线`、上标数字）不会错位；
+ 表格用 pdfplumber：按格线切单元格，跨页续行才能并回同一条记录。

本机没有 python/py，需这样运行：
    uv run --no-project --with pymupdf --with pdfplumber python pdf_to_md.py \
        <pdf> <输出.md> --title "文件标题" --source "> 来源引用块"

生成后仍需人工补元信息、核对附件标题层级，并做逐字比对（脚本只保证「不漏字、不错序」）。
"""

import argparse
import re
from pathlib import Path

import pdfplumber
import pymupdf

# 单元格内的条目编号：只有它们才代表「新的一条」，其余换行都是排版折行
ITEM_MARKS = "①②③④⑤⑥⑦⑧⑨⑩⑪⑫⑬⑭⑮⑯⑰⑱⑲⑳"
# 段落流里的表格占位符
TABLE_SENTINEL = "\x00TABLE\x00"


def parse_pages(spec: str | None) -> list[int] | None:
    """解析 `--table-pages`：支持 `7-10` 与 `7,8,9` 两种写法。"""
    if not spec:
        return None
    pages: list[int] = []
    for part in spec.split(","):
        part = part.strip()
        if "-" in part:
            start, end = part.split("-", 1)
            pages.extend(range(int(start), int(end) + 1))
        elif part:
            pages.append(int(part))
    return pages


def pick_table(page: pdfplumber.page.Page):
    """挑出页面上真正的表格。

    `find_tables()` 会把「多行、整段等宽排版的正文」误判成单列表格，
    所以要求至少有一个 2 列以上的行才认（跨页续接行可能只有一个非空单元格，
    但列数仍是多列，故这里看列数、不看非空单元格数）。
    """
    for table in page.find_tables():
        data = table.extract()
        if not data:
            continue
        columns = max(len(row) for row in data)
        if columns >= 2:
            return table
    return None


def find_table_boxes(pdf: pdfplumber.PDF, pages: list[int] | None) -> dict[int, tuple]:
    """找出每页表格的 bbox（1 起页码）；未指定页时扫描全书。"""
    numbers = pages if pages else range(1, len(pdf.pages) + 1)
    boxes: dict[int, tuple] = {}
    for page_no in numbers:
        table = pick_table(pdf.pages[page_no - 1])
        if table is not None:
            boxes[page_no] = table.bbox
    return boxes


def page_lines(page: pymupdf.Page, box: tuple | None) -> tuple[list[str], list[str]]:
    """返回（表格上方、表格下方）两段正文——表格可能只占页面的上半或下半。"""
    width, height = page.rect.width, page.rect.height
    if box is None:
        return page.get_text("text", sort=True).splitlines(), []
    top, bottom = box[1], box[3]
    above = page.get_text("text", sort=True, clip=pymupdf.Rect(0, 0, width, top))
    below = page.get_text("text", sort=True, clip=pymupdf.Rect(0, bottom, width, height))
    return above.splitlines(), below.splitlines()


def is_page_number(line: str) -> bool:
    """页码行：整行只有数字（附件页码常从 1 重新开始，故不能按页码范围判断）。"""
    return line.strip().isdigit()


def build_paragraphs(lines: list[str]) -> list[str]:
    """按行首缩进切段：缩进 >= 2 空格开新段，其余续接当前段。"""
    paragraphs: list[str] = []
    current = ""
    for line in lines:
        if line == TABLE_SENTINEL:
            if current:
                paragraphs.append(current)
            paragraphs.append(TABLE_SENTINEL)
            current = ""
            continue
        if not line.strip() or is_page_number(line):
            continue
        body = line.strip()
        if body == "附件":
            # 附件页的「附件」二字居中，没有缩进，需特判成独立段落
            if current:
                paragraphs.append(current)
            paragraphs.append(body)
            current = ""
        elif line.startswith("  "):
            if current:
                paragraphs.append(current)
            current = body
        else:
            current += body
    if current:
        paragraphs.append(current)
    return paragraphs


def table_rows(pdf: pdfplumber.PDF, boxes: dict[int, tuple]) -> list[list[str]]:
    """抽取表格，并把被分页切开的续行并回上一条记录。"""
    rows: list[list[str]] = []
    for page_no in sorted(boxes):
        table = pick_table(pdf.pages[page_no - 1])
        if table is None:
            continue
        for row in table.extract():
            cells = [(cell or "").replace("\r", "") for cell in row]
            if not any(c.strip() for c in cells):
                continue
            # 序号、风险环节列都为空 → 本页续接的上一条记录
            if not cells[0].strip() and not cells[1].strip() and rows:
                for idx in range(2, len(cells)):
                    starter = cells[idx].lstrip()[:1]
                    sep = "\n" if starter and starter in ITEM_MARKS else ""
                    rows[-1][idx] += sep + cells[idx]
                continue
            rows.append(cells)
    return rows


def cell_to_markdown(cell: str) -> str:
    """单元格转 Markdown：①②③ 前换行用 <br> 分条，其余折行直接去掉。"""
    text = cell
    for mark in ITEM_MARKS:
        text = text.replace("\n" + mark, "<br>" + mark)
    return text.replace("\n", "").strip()


def render_table(rows: list[list[str]]) -> list[str]:
    """渲染成 Markdown 表格（首行为表头）。"""
    header = rows[0]
    out = ["| " + " | ".join(header) + " |"]
    out.append("| " + " | ".join(["---"] * len(header)) + " |")
    for row in rows[1:]:
        out.append("| " + " | ".join(cell_to_markdown(c) for c in row) + " |")
    return out


def heading_prefix(paragraph: str, after_attachment: bool) -> str | None:
    """判定标题层级；条款序号（`1.`、`（1）`）保持普通段落，避免被解析成列表。"""
    if re.match(r"^[一二三四五六七八九十]+、", paragraph):
        return "## "
    if re.match(r"^（[一二三四五六七八九十]+）", paragraph):
        return "### "
    if paragraph == "附件":
        return "## "
    if after_attachment:
        return "### "
    return None


def main() -> None:
    parser = argparse.ArgumentParser(description="文本可读 PDF → Markdown 章节草稿")
    parser.add_argument("pdf", type=Path)
    parser.add_argument("out", type=Path)
    parser.add_argument("--title", required=True, help="一级标题（文件标题）")
    parser.add_argument("--source", default="", help="首行来源引用块，如 `> 此文件转写自……`")
    parser.add_argument("--table-pages", dest="table_pages", help="表格页，如 `7-10` 或 `7,8`")
    args = parser.parse_args()

    doc = pymupdf.open(args.pdf)
    with pdfplumber.open(args.pdf) as pdf:
        boxes = find_table_boxes(pdf, parse_pages(args.table_pages))
        rows = table_rows(pdf, boxes)

    raw_lines: list[str] = []
    first_table_page = min(boxes) if boxes else None
    for idx, page in enumerate(doc, start=1):
        box = boxes.get(idx)
        if box is None:
            raw_lines.extend(page.get_text("text", sort=True).splitlines())
            continue
        above, below = page_lines(page, box)
        raw_lines.extend(above)
        if idx == first_table_page:
            raw_lines.append(TABLE_SENTINEL)  # 表格插在这一页正文之后
        raw_lines.extend(below)

    paragraphs = build_paragraphs(raw_lines)
    # 首页标题单独写在开头，正文里去掉重复的那一行
    if paragraphs and paragraphs[0] == args.title:
        paragraphs.pop(0)

    blocks: list[str] = []
    if args.source:
        blocks += [args.source, ""]
    blocks += ["# " + args.title, ""]

    after_attachment = False
    for paragraph in paragraphs:
        if paragraph == TABLE_SENTINEL:
            blocks += render_table(rows)
            blocks.append("")
            continue
        prefix = heading_prefix(paragraph, after_attachment)
        after_attachment = paragraph == "附件"
        blocks += [(prefix or "") + paragraph, ""]

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(blocks).rstrip("\n") + "\n", encoding="utf-8")
    print(f"wrote {args.out}（{len(paragraphs)} 段，表格 {len(rows) - 1 if rows else 0} 行）")


if __name__ == "__main__":
    main()

