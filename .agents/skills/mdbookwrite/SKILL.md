---
name: mdbookwrite
description: 编写与修改本仓库 mdbooks/ 下 mdbook 工程里的 Markdown 章节。当用户要求新增或编辑某本 mdbook 书的一篇 .md、处理文件内以 #AI 开头的编写任务、把章节登记到 src/SUMMARY.md、或新增一本书并在 mdbooks/manifest.toml 登记时触发。涵盖工程结构、MathJax 数学公式、可运行 Rust 代码块、#AI 任务约定与本地构建 / 预览流程。
---

# mdbookwrite —— 编写单个 mdbook 文件

指导 AI 在本项目（个人静态主页，Rust + Dioxus）里编写 / 修改 `mdbooks/` 下 mdbook 工程的 Markdown 章节。

## 何时使用

- 新增或编辑某本 mdbook 书里的一篇章节 `.md`（`mdbooks/<书名>/src/**/*.md`）
- 处理文件内以 `#AI` 开头的**编写任务**（见「`#AI` 任务约定」）
- 新增一本书：建 `book.toml` / `src/SUMMARY.md`，并在 `mdbooks/manifest.toml` 登记
- 把新章节登记进 `src/SUMMARY.md`

## 工程结构

每本书是一个目录（`mdbooks/<书名>/`）：

```
mdbooks/
├── manifest.toml              # 书籍清单：[[book]] 的 name / path / id / intro
└── <书名>/
    ├── .gitignore             # 内容为一行 `book`，忽略构建产物目录
    ├── book.toml              # 书名 / 作者 / 语言 / 输出配置
    ├── book/                  # mdbook build 产物（已被 gitignore）
    └── src/
        ├── SUMMARY.md         # 目录：登记每一章
        ├── 关于此书.md
        └── <分组>/<章节>.md   # 章节正文
```

`book.toml` 需开启数学支持（本仓库书籍均如此）：

```toml
[book]
title = "算法"
authors = ["matt8848-max"]
language = "zh_cn"

[output.html]
mathjax-support = true
```

## 单个章节 md 的写作约定

### 位置与标题层级

- 章节放在 `src/` 下，可按主题建子目录（如 `src/使用场景/算法的基本概念.md`），文件名用中文即可。
- 标题层级：`#` 章标题、`##` 节、`###` 小节；文件首行写 `# 章节名`。

### 登记到 SUMMARY.md

新增章节后**必须登记**，否则不会出现在侧边栏。分组用 `# 分组名`，条目用 `- [显示名](./相对路径.md)`：

```markdown
# Summary

- [关于此书](./关于此书.md)

# 使用场景

- [算法的基本概念](./使用场景/算法的基本概念.md)
```

路径是相对 `src/`、以 `./` 开头的相对路径。

### 数学公式（MathJax）

前提：`book.toml` 里 `mathjax-support = true`。**转义规则是本仓库最容易踩的坑**：

- 行内公式写成 `\\( ... \\)`，行间公式写成 `\\[ ... \\]`。
  原因：「反斜杠 + 标点」（`\(`、`\)`、`\[`、`\]`、`\{`、`\}`）在 Markdown 里会被吃掉一个反斜杠，所以要写成**双反斜杠**。
- 「反斜杠 + 字母」的命令（`\sum`、`\frac`、`\ln`、`\begin`、`\end`、`\times`、`\Theta` …）用**单个**反斜杠即可。
- 多行对齐用 `\begin{align} ... \end{align}`，行尾换行写 **4 个**反斜杠，对齐点用 `&`：

```markdown
\begin{align}
E[X]&=E[\sum_{i=1}^{n} X_i]\\\\
&=\sum_{i=1}^{n} E[X_i]\\\\
&=\sum_{i=1}^{n} \frac{1}{i}\\\\
&=\ln n + O(1)
\end{align}
```

- 分段函数用 `\begin{cases} ... \end{cases}`，每行结尾同理写 4 个反斜杠，条件用 `&` 对齐。
- 字面花括号写成 `\\{ ... \\}`（如 `I\\{A\\}`）。

### 可运行的 Rust 代码块

- 用 rust 围栏；mdbook v0.5.4 会把它渲染成**可运行 playground**（产物为 `<pre class="playground"><code class="language-rust">`）。
- 示例代码遵循 rustweb 规范：**中文注释**，内部注释侧重解释「为什么」。
- 想让代码可跑需要 `fn main`；若只想展示函数本体，用**隐藏行**——以 `# `（井号 + 空格）开头的行会参与编译 / 运行，但渲染时被隐藏：

````markdown
```rust
fn hire_assistant(assistants: &Vec<usize>) {
    // ...
}

# fn main() {
#     hire_assistant(&vec![3, 1, 4]);
# }
```
````

### 其他排版

- 无序列表用 `+`；表格用标准 Markdown 表格（仓库正文大量使用，便于对照复杂度）。
- 外链用 `[标题](url)`。

## `#AI` 任务约定

**以 `#AI` 开头的行是留给 AI 的编写任务标记**（`#` 与 `AI` 之间无空格，因此不会被 Markdown 当成标题）。它可能出现在正文，也可能出现在 rust 代码块内部。处理方式：

1. 读懂该行要求，就地把标记替换为符合上下文的正式内容（代码 / 公式 / 文字）。
2. 保持所在语境的行文风格与**转义规则**（尤其数学公式）。
3. 完成后**不保留** `#AI` 标记行。
4. 若任务落在代码块里，补全后代码应当可运行（必要时用隐藏的 `main`）。

示例（代码块内的任务）：

````markdown
```rust
#AI，此处编写一个单层循环的示例
```
````

→ 替换为一段真正的单层循环代码（如单遍历求最值）。

## 本地构建与预览

单本书构建（在仓库根目录执行）：

```bash
mdbook build mdbooks/<书名> --dest-dir <临时目录>
```

- 本地 / CI 的 mdbook 版本均为 **v0.5.4**（用 `mdbook --version` 核对）。
- 校验用 `mdbook test mdbooks/<书名>`：它会编译所有 rust 代码块（含隐藏行），能跑通说明示例代码可用。
- 站点集成由 `webpage/build.rs` 的 `run_mdbook()` 完成：调用 `mdbook build <book_dir> --dest-dir public/books/<id>`，再给产物 HTML 注入「返回主页 / 返回知识库」按钮。
- 整站预览：`cd webpage; dx build --release --platform web`，产物在 `target/dx/webpage/release/web/public/books/<id>/`，打开 `#/knowledge` 看知识库卡片。
- `mdbook` 不在 PATH 时，`build.rs` 仅告警并跳过 mdbook 书籍。

## 新增一本书

1. 建目录 `mdbooks/<书名>/`，写 `book.toml`（含 `mathjax-support = true`）与 `.gitignore`（一行 `book`）。
2. 建 `src/SUMMARY.md` 与至少一篇 `src/*.md`。
3. 在 `mdbooks/manifest.toml` 追加：

```toml
[[book]]
name = "书名"
path = "<目录名>"
id = "<ascii-短名>"
intro = "一句话简介"
```

4. `id` 用 ASCII 短名（中文目录名自动推导的别名不稳定，会导致线上 URL 变化）。

## 常见坑

1. **数学转义**：`\(`、`\{` 等「反斜杠 + 标点」写双反斜杠；`\sum` 等「反斜杠 + 字母」写单个；`align` / `cases` 行尾换行写 4 个反斜杠。写错会导致公式不渲染或乱码。
2. **忘登记 SUMMARY**：新章节不会出现在目录里。
3. **`book/` 是产物目录**：已 gitignore，勿提交，也勿手工改其中文件。
4. **`#AI` 标记**：处理完即删除，不要留在成品里。
5. **改了 `book.toml` / 章节后**：重新 `dx build`（会重新调用 mdbook）。

## 参考

- 范例章节：`mdbooks/Algorithm/src/使用场景/算法的基本概念.md`
- 站点集成：`webpage/build.rs`（`build_mdbooks` / `run_mdbook`）、`CLAUDE.md`「内容管线」章节
- 注释 / 测试规范：`rustweb` skill 的 `docs/coding-standards.md`
