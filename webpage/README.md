# webpage

个人主页的 Dioxus（web/wasm）前端。

## 结构

```
webpage/
├── Dioxus.toml        # 应用配置；application.public_dir = "public"
├── build.rs           # 编译期：汇总内容源，生成静态页与数据模块
├── build/util.rs      # 构建脚本的纯逻辑（标识生成 / HTML 注入 / 文章模板）
├── public/            # 生成物（gitignore）：books/ 下的书籍静态页与 mdbook 产物
├── src/
│   ├── main.rs        # 启动入口（LaunchBuilder + HashHistory）
│   ├── app.rs         # 根组件（图标 + 基础样式 + Router）
│   ├── route.rs       # Route 枚举（/ 、/knowledge、/games、兜底 404）
│   ├── data.rs        # include!(OUT_DIR)：书籍/游戏数据 + 检索逻辑
│   ├── pages/         # home / knowledge / games / not_found
│   └── components/    # book_card 等可复用组件
├── tests/build_util.rs# 构建脚本纯逻辑的集成测试
└── assets/            # base.css + 每页一个 CSS + favicon.ico
```

## 内容从哪来

内容源在仓库根：`books/`（单篇 Markdown）、`mdbooks/`（mdbook 工程）、`games/`（一个游戏一个 TOML）。
`build.rs` 在编译期把它们转成 `public/books/**`（静态页，含左上角浮动「返回主页」按钮），
并生成 `OUT_DIR/books.rs`、`OUT_DIR/games.rs` 供前端 `include!`。

`build.rs` 需要 PATH 中有 `mdbook`（本地 v0.5.4）。

## 常用命令

```bash
cargo check                       # 类型检查
cargo test                        # 测试
cargo clippy --all-targets -- -D warnings
cargo fmt
dx build --release --platform web # 生产构建
```

预览：

```bash
uv run --no-project python -m http.server 8088 \
  --directory target/dx/webpage/release/web/public
```
