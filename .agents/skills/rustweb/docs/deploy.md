---
title: "Dioxus Web 构建与 GitHub Pages 部署"
---

# Dioxus Web → GitHub Pages

## 版本基线

| 组件 | 版本 |
|---|---|
| dioxus | 0.7.1 |
| dioxus-cli（`dx`） | 0.7.10 |
| rustc / cargo | 1.98.0 |
| 编译目标 | wasm32-unknown-unknown |

## 本地构建

```bash
cd <crate 根>
dx build --release --platform web
```

产物目录：`target/dx/<package.name>/release/web/public`
其中 `<package.name>` 取自 `Cargo.toml` 的 `package.name`，**不是目录名**。

### 产物结构（默认模板实测）

```
public/
├── index.html                                   # 仅含 <div id="main"> 与 <script type="module">
└── assets/
    ├── <pkg>-dxh<hash>.js                       # 胶水 JS（约 55 KB）
    ├── <pkg>_bg-dxh<hash>.wasm                  # wasm 本体（约 1.9 MB）
    ├── main-dxh<hash>.css                       # Rust 侧 asset!() 引入的样式
    ├── tailwind-dxh<hash>.css                   # dx 调用 tailwind 生成
    ├── header-dxh<hash>.svg
    └── favicon-dxh<hash>.ico
```

要点：`index.html` 里**只出现 JS**（形如 `/./assets/<pkg>-dxh<hash>.js`）。CSS 与图片是由 **wasm 运行时注入**的，其真实文件名藏在 wasm 二进制内 —— 排查线上资源是否存在时，需从 wasm 中解析，不能靠猜。

## dx 的隐式依赖

`dx` 会自动下载并缓存以下工具，**无需在 CI 中单独安装**：

- `wasm-bindgen` —— 若预编译包下载失败（如 GitHub Releases 不可达），会**回退到源码编译**，本地约多花 60 秒；CI 上通常正常
- `wasm-opt`、`esbuild`、`tailwindcss` —— 独立二进制；Tailwind 的产物会写入项目的 `assets/tailwind.css`

## GitHub Pages 部署配方

```yaml
permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages

jobs:
  build:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: <应用子目录>        # 应用位于仓库子目录时
    steps:
      - uses: actions/checkout@v7
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: <应用子目录> -> target
      - uses: taiki-e/install-action@v2        # 走 binstall，实测 3 秒装好 dx
        with:
          tool: dioxus-cli@0.7.10
      - run: dx build --release --platform web
      - uses: actions/configure-pages@v6
      - uses: actions/upload-pages-artifact@v5
        with:
          path: <应用子目录>/target/dx/<package.name>/release/web/public   # 仓库根相对路径
  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - id: deployment
        uses: actions/deploy-pages@v5
```

前置条件：仓库 `Settings → Pages → Source` 必须选择 **GitHub Actions**，否则 `configure-pages` 会报 “Get Pages site failed”。

## 常见坑

1. **资源 hash 跨环境不一致**。同一份源码在本地与 CI 上产出的 `-dxh<hash>` 不同（内容哈希受构建环境影响）。因此**不要用本地算出的文件名去探测线上文件** —— 会得到假 404。正确做法：沿线上 `index.html` → JS → wasm 逐层解析出真实文件名。
2. **`base_path` 取值取决于站点类型**：
   - 用户/组织根站点（`<user>.github.io`）：保持默认 `"."`，资源路径为 `/./assets/...`
   - 项目站点（`<user>.github.io/<repo>`）：必须设为 `"/<repo>"`，否则根路径资源全部 404
3. **应用位于子目录时**：`defaults.run.working-directory` 用子目录，但 `upload-pages-artifact` 的 `path` 必须写**仓库根相对**路径。
4. **子目录内残留 `.git`**：会让该目录在父仓库中变成 gitlink（submodule），CI `checkout` 后目录为空，构建必然失败。为位于子目录的应用搭建 CI 前，先确认其内部没有独立的 `.git`。
5. **SPA 直链刷新**：GitHub Pages 不做 SPA 回退。若使用 `WebHistory`（干净 URL），刷新 `/some/path` 会 404；对策是构建后 `cp index.html 404.html`（代价：直链首次响应状态码为 404）。若可接受 URL 带 `#`，直接用 `HashHistory` 可完全规避此问题。
