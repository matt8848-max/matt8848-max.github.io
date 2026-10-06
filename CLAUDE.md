# CLAUDE.md

本文件为 AI 编码助手提供本仓库的上下文与约束。

## 项目概览

- **目标**：个人静态主页，托管于 GitHub Pages
- **线上地址**：https://matt8848-max.github.io/
- **技术栈**：Rust + Dioxus 0.7.1（web/wasm 端，含 `router`）+ 手写 CSS（每页一个 CSS 文件）
- **页面**：主页（按钮入口）→「知识库」「游戏评价」；未知路由渲染 404 页
- **路由**：使用 `HashHistory`（`src/main.rs` 的 `LaunchBuilder`），规避 GitHub Pages 无 SPA 回退的问题
- **代码位置**：应用代码在 `webpage/`（crate 根）；仓库根目录放内容源（`books/` `mdbooks/` `games/`）与 CI 文档

## 仓库结构

```
persona/
├── .agents/skills/rustweb/       # Rust/Web 开发规范知识库（编码 / 测试 / 工具链 / 部署）
├── .agents/skills/addgame/       # 新增游戏评价的操作规范（可用 `addgame` skill 加载）
├── .github/workflows/deploy.yml  # 构建并部署到 GitHub Pages
├── books/                        # 短文集：*.md + manifest.toml（名称/路径/简介）
├── mdbooks/                      # mdbook 工程：<书名>/book.toml + manifest.toml
├── games/                        # 游戏评价（一个游戏一个 *.toml）
├── webpage/                      # Dioxus 应用（crate 根，勿改名）
│   ├── Cargo.toml                # package.name = "webpage"，CI 产物路径依赖它
│   ├── Dioxus.toml               # 不含 base_path；application.public_dir = "public"
│   ├── build.rs                  # 汇总 books/ mdbooks/ games/，生成静态页与数据模块
│   ├── build/util.rs             # 构建脚本的纯逻辑（被 tests/build_util.rs 复用测试）
│   ├── public/                   # build.rs 生成物（gitignore）：书籍静态页 / mdbook 产物
│   ├── src/                      # main / app / route / data / tags + pages/ + components/
│   ├── tests/build_util.rs       # 构建脚本纯逻辑的集成测试
│   └── assets/                   # base.css + 每页一个 CSS（home/knowledge/games/not_found）+ favicon.ico
└── 任务清单.md
```


## 常用命令

| 目的 | 命令 |
|---|---|
| 类型检查（最快） | `cargo check --manifest-path webpage/Cargo.toml` |
| 运行测试 | `cargo test --manifest-path webpage/Cargo.toml` |
| 代码质量 | `cargo clippy --manifest-path webpage/Cargo.toml -- -D warnings` |
| 格式化 | `cargo fmt --manifest-path webpage/Cargo.toml` |
| 生产构建 | `cd webpage; dx build --release --platform web` |
| 本地预览 | `uv run --no-project python -m http.server 8088 --directory webpage/target/dx/webpage/release/web/public` |

本机没有 `python` / `py` / `node`，起静态服务必须用 `uv run --no-project python`。
`dx build` / `dx serve` 会触发 `build.rs`，它需要 PATH 中有 `mdbook`（本地 MDbook v0.5.4，CI 由 `taiki-e/install-action` 安装 `mdbook@0.5.4`）。缺 `mdbook` 时仅告警、跳过 mdbook 书籍。

## 内容管线（书籍 / 游戏）

- 内容源在仓库根：`books/`（单篇 `.md`）、`mdbooks/`（mdbook 工程）、`games/`（一个游戏一个 `.toml`）
- 两个书籍文件夹各有一个 `manifest.toml`，用 `[[book]]` 记录 `name` / `path` / `intro` /（可选）`id`
- 游戏标签清单（8 个）**硬编码**在 `webpage/src/tags.rs` 的 `GAME_TAGS`；**`games/*.toml` 的 `tags` 必须取自它**
- `webpage/build.rs` 在编译期：
  - 用 `pulldown-cmark` 把 `books/*.md` 转成静态 HTML → `webpage/public/books/<id>.html`
  - 调 `mdbook build --dest-dir` 把 mdbook 输出到 `webpage/public/books/<id>/`
  - 给上述所有 HTML **注入右下角浮动导航按钮组「返回主页」「返回知识库」**（`build/util.rs` 的 `inject_nav_buttons`；知识库入口为哈希路由 `/#/knowledge`，主页为 `/`）
  - 汇总成 `OUT_DIR/books.rs` / `OUT_DIR/games.rs`，由 `src/data.rs` 用 `include!` 引入
  - `games.rs` 生成 `GAMES`（游戏条目）；标签清单硬编码在 `src/tags.rs`，合法性由测试 `test_game_tags_should_cover_all_games` 保证
- **静态资源如何进产物**：`Dioxus.toml` 的 `application.public_dir = "public"`（dx 默认值）。dx 会把 `webpage/public/` 整棵递归复制进站点根，**且不 hash 文件名 / 不改变相对结构**，因此 mdbook 内部相对链接与预先命名的资源（如 `css/general-xxxx.css`）都保持有效
- 页面对书籍的链接使用站点根绝对路径（`/books/<id>.html`、`/books/<id>/index.html`），点击即整页跳转到静态页


## 新增游戏条目

游戏评价的增改规则见 `.agents/skills/addgame/`（可用 `addgame` skill 加载），要点：

- 一个游戏一个文件：`games/<id>.toml`（`<id>` 为 ASCII 短名）；**文件名即列表排序依据**（`build.rs` 按文件名字典序）
- 字段：`name`（必填）/ `platform` / `status` / `score`（10 分制）/ `tags` / `review`；**没有 `played_at`**
- `tags` 必须取自 `webpage/src/tags.rs` 硬编码的 8 个标签之一，**该 8 分类已锁定、不再新增**；更通俗的玩法分类（RTS、4X、战棋等）写进 `review`
- 各标签含义见 `games/README.md`「樱井政博对 8 个标签的讲解」
- 校验流程：`cargo test`（`test_game_tags_should_cover_all_games` 校验标签合法性）→ `clippy` / `fmt` → `dx build` → 本地预览 `#/games`


## 构建与部署流水线

- 工作流：`.github/workflows/deploy.yml`，`push main` 或手动 `workflow_dispatch` 触发
- 步骤：checkout → 装 wasm32 工具链 → `Swatinem/rust-cache` → `taiki-e/install-action` 装 `dioxus-cli@0.7.10` → `taiki-e/install-action` 装 `mdbook@0.5.4` → `dx build --release --platform web` → `configure-pages` → `upload-pages-artifact` → `deploy-pages`
- **产物路径（硬编码在 CI 中）**：`webpage/target/dx/webpage/release/web/public`
  其中的 `webpage` 来自 `Cargo.toml` 的 `package.name`。**重命名 crate 必须同步修改 `deploy.yml` 的 `path`。**
- 仓库 `Settings → Pages → Source` 必须为 **GitHub Actions**
- 实测（本仓库 ubuntu-latest）：
  - 首次（无缓存）：全程约 80 秒，其中 `dx build` 48s
  - 缓存命中（`Swatinem/rust-cache`）：全程约 53 秒，其中 `dx build` 15s；装 `dx` 约 2s

## 关键约束 / 已踩过的坑

1. **不要给 `Dioxus.toml` 加 `base_path`**。本站是用户根站点（`<user>.github.io`），默认 `"."` 时资源路径为 `/./assets/...`，线上已验证可用；设置反而会 404。
2. **不要用本地构建的资源 hash 去探测线上文件**。同一份源码在本地与 CI 生成的 `-dxh<hash>` 不同（实测本地 JS `webpage-dxh66d666ec6ba235e.js`，线上 `webpage-dxh9cd97115aeea9ed.js`）；但 CI 多次构建之间是可复现的。核对线上资源应从线上的 `index.html` → JS → wasm 逐层解析实际文件名（CSS / 图标的名字只存在于 wasm 二进制内）。
3. **`webpage/` 目录内不允许存在 `.git`**。它历史上是一个嵌套仓库，会被当作 gitlink（submodule），导致 CI checkout 后目录为空、构建失败；已删除，不要重新 `git init`。
4. Dioxus 资源引用一律使用绝对路径：`asset!("/assets/xxx.css")`。
5. Windows 下不要依赖 `cd dir && cargo ...` 前缀（中文路径会导致 `cd` 失效），统一用 `--manifest-path` 或绝对路径。
6. **不要给 `Dioxus.toml` 的 `application.public_dir` 设成 `assets/` 或相对站点根的路径以外的值**：`public_dir` 目录会被整棵复制到站点根，且**不做 hash**。构建期生成的书籍静态页必须放这里，才能既保留 mdbook 的文件名又让相对链接生效。
7. `webpage/public/` 是**生成物**（已在 `.gitignore` 中忽略），不要手工放文件进去（会被 `build.rs` 的清理逻辑覆盖 `public/books/`）。
8. 内容清单 `books/manifest.toml` / `mdbooks/manifest.toml` 的 `id` 建议显式给出 ASCII 短名：中文文件名自动推导出的别名不稳定，会导致线上 URL 变化。
9. **新增 / 修改游戏条目时，`tags` 只能取 `webpage/src/tags.rs` 硬编码的 8 个标签，禁止自造标签。** 该 8 分类**已锁定，不再新增**——更通俗的玩法分类（RTS、4X、战棋等）写进 `review` 正文。合法性由 `src/tags.rs` 的测试 `test_game_tags_should_cover_all_games` 保证。

## 编码规范

遵循 `.agents/skills/rustweb/` 下的规范（可用 `rustweb` skill 加载）：

- 中文注释；内部注释说明**为什么**；公开项写文档注释；文档注释中 `<xxx>` 之类内容用反引号包裹，避免 rustdoc 告警
- 模块单一职责，按功能拆分，公用逻辑下沉到独立模块
- 依赖一律用 `cargo add` 添加，不手改 `Cargo.toml`
- 单元测试与源码同目录（`#[cfg(test)]`），命名 `test_<模块>_should_<预期行为>`，覆盖 happy path / 边界 / 错误路径
- Dioxus 0.7：hook 只能在组件函数体顶层调用，禁止在闭包内调用；RSX `for` 循环体内不能写 `let`；元素属性 `type` 写作 `r#type`

## 开发流程

1. `cargo check` 快速验证类型
2. 编写 / 更新测试并 `cargo test`
3. `cargo clippy -- -D warnings` 清零告警
4. `dx build --release --platform web` 本地生产构建
5. 必要时用 `uv run --no-project python -m http.server 8088` 起服务人工验证
6. `git push origin main`，确认 Actions 绿色且线上可访问
