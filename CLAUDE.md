# CLAUDE.md

本文件为 AI 编码助手提供本仓库的上下文与约束。

## 项目概览

- **目标**：个人静态主页，托管于 GitHub Pages
- **线上地址**：https://matt8848-max.github.io/
- **技术栈**：Rust + Dioxus 0.7.1（web/wasm 端）+ Tailwind CSS v4（由 dx 自动获取）
- **需求文档**：`构建设想.md` —— 主页为按钮入口，下设「游戏个人评价」「学习笔记」「个人小项目列表」
- **代码位置**：应用代码在 `webpage/`（crate 根）；仓库根目录只放 CI 与文档

## 仓库结构

```
persona/
├── .agents/skills/rustweb/       # Rust/Web 开发规范知识库（编码 / 测试 / 工具链 / 部署）
├── .github/workflows/deploy.yml  # 构建并部署到 GitHub Pages
├── webpage/                      # Dioxus 应用（crate 根，勿改名）
│   ├── Cargo.toml                # package.name = "webpage"，CI 产物路径依赖它
│   ├── Dioxus.toml               # 不含 base_path（用户根站点，走默认 "."）
│   ├── src/main.rs
│   └── assets/                   # main.css / tailwind.css / header.svg / favicon.ico
└── 构建设想.md
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

## 构建与部署流水线

- 工作流：`.github/workflows/deploy.yml`，`push main` 或手动 `workflow_dispatch` 触发
- 步骤：checkout → 装 wasm32 工具链 → `Swatinem/rust-cache` → `taiki-e/install-action` 装 `dioxus-cli@0.7.10` → `dx build --release --platform web` → `configure-pages` → `upload-pages-artifact` → `deploy-pages`
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
