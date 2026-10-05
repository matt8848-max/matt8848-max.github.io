---
title: "Rust 编程语言"
---

# Rust

系统级编程语言，强调内存安全与零成本抽象。用于 CLI 工具开发、高性能后端。

## 核心能力
- 所有权与借用系统、生命周期标注
- 泛型、trait 与 trait 对象
- 宏体系（声明宏与过程宏）
- 异步编程（tokio / async-std）
- 错误处理（anyhow / thiserror）

## 模块组织

使用 Rust 2018 的扁平模块系统，每个模块一个文件。项目结构示例：

```
src/
├── lib.rs          ← 统一导出所有模块
├── main.rs         ← 只调用 lib 入口
├── cli.rs          ← 命令分发（clap 定义）
├── store.rs        ← 配置管理（Config 结构体）
├── store/          ← 子模块目录
│   └── fs.rs       ← 公用的文件系统操作
├── model.rs        ← 数据模型
├── api.rs          ← 接口层
└── service.rs      ← 业务逻辑
```

关键规则：

**1. `lib.rs` 统一导出所有模块** — main.rs 只需调用 lib 入口：

```rust
// lib.rs — 统一管理模块导出
pub mod api;
pub mod cli;
pub mod model;
pub mod service;
pub mod store;

// main.rs — 尽量薄，只做入口分发
mod lib;
#[tokio::main]
async fn main() {
    lib::run().await;
}
```

**2. 文件夹子模块** — 公用工具代码独立成 `parent.rs` + `parent/child.rs`（禁止 `mod.rs`）：

```rust
// store.rs — 模块主文件，声明子模块
pub mod fs;  // 声明 store/fs.rs 作为子模块

// store/fs.rs — 被 store.rs 和其他模块共享的文件操作
use anyhow::Result;
use std::path::Path;

/// 读取文件内容（所有模块通过 store::fs::read_file 调用）
pub fn read_file(path: &Path) -> Result<String> { ... }
/// 写入文件内容（自动创建父目录）
pub fn write_file(path: &Path, content: &str) -> Result<()> { ... }
/// 删除文件
pub fn delete_file(path: &Path) -> Result<()> { ... }
```

**3. 复用优先于重复** — 多个模块用到的文件操作统一走 `store::fs`：

```rust
// service.rs — 删除文件时复用 store/fs.rs 的 delete_file
pub fn remove_document(path: &Path) -> Result<()> {
    crate::store::fs::delete_file(path)  // 不自己实现删除逻辑
}
```

## 文档注释规范

### HTML 标签转义

rustdoc 会将文档注释中的 `<tag>` 解析为 HTML 标签。若注释中出现 `<xxx>.md`、`<value>` 等尖括号写法，会触发 `unclosed HTML tag` 警告（`rustdoc::invalid_html_tags`）。

**正确做法**：用反引号将含尖括号的内容包成行内代码，rustdoc 不会解析行内代码中的内容。

```rust
// 错误：rustdoc 将 <name> 视为未闭合的 HTML 标签，产生 warning
/// 条目名称（存储为 <name>.md，后续通过此名称编辑/删除）。

// 正确：反引号包裹成行内代码，rustdoc 不解析其中的内容
/// 条目名称（存储为 `<name>.md`，后续通过此名称编辑/删除）。
```

**排查方法**：该警告通常由 `cargo doc` 的 `rustdoc::invalid_html_tags` 触发，多出现在文档注释中的占位符/文件名描述。写文档注释时涉及 `<...>` 形式的内容一律加反引号。

## 日志方案

Rust 使用 `log` 门面 + 具体实现的模式：

```bash
# 项目级依赖
cargo add log          # log 门面：提供 info! / warn! / error! / debug! 宏
cargo add env_logger   # 终端输出实现，通过 RUST_LOG 控制级别
```

**初始化** — 在 main.rs 的最顶部调用一次：

```rust
#[tokio::main]
async fn main() {
    env_logger::init();  // 必须在所有操作之前初始化
    // ... 业务逻辑
}
```

**分级使用** — 各模块按场景选择日志级别：

```rust
// 用户可见的关键状态变化（如操作成功、流程节点）
log::info!("构建成功: {}", output_path.display());

// 详细调试信息（文件操作细节、解析过程）
log::debug!("加载配置文件: {}", path.display());
log::debug!("文档生成完成 ({} 字符)", content.len());

// 错误信息（替代 eprintln!）
log::error!("处理失败: {}", err);
```

**运行控制** — 通过环境变量开关日志：

```bash
# 默认：不输出日志，仅 console 可见
myapp list

# info 级别：查看关键操作流程
RUST_LOG=info myapp run

# debug 级别：查看所有文件操作细节
RUST_LOG=debug myapp list
```

## 跨平台条件编译（Web/桌面差异）

同一份代码同时编译为 wasm（Web）与 native（桌面）时，平台差异必须通过
`#[cfg(target_arch = "wasm32")]` 条件编译封装成内部实现细节，
外部调用方使用**统一接口**，无需感知平台。

**模式**：公共函数不带 cfg，内部按平台分支实现。

```rust
// 统一入口（无 cfg）
pub(crate) async fn http_put(base: &str, path: &str, body: Option<serde_json::Value>)
    -> Result<serde_json::Value, String> {
    http_request("PUT", base, path, body).await
}

// wasm：gloo-net
#[cfg(target_arch = "wasm32")]
async fn http_request(...) -> Result<serde_json::Value, String> { /* gloo_net */ }

// native：reqwest
#[cfg(not(target_arch = "wasm32"))]
async fn http_request(...) -> Result<serde_json::Value, String> { /* reqwest */ }
```

**适用场景**：
- HTTP 客户端（wasm 用 gloo-net，native 用 reqwest）
- 定时器（wasm 用 gloo-timers，native 用 tokio）
- 本地时间/时区（wasm 用 JS Date，native 用 chrono::Local）
- 原生提示（wasm 用 `window.alert`，native 用 UI 内 Toast 浮层）

**原则**：UI 组件保持平台无关；一切平台差异收敛到 cfg 分支的底层函数。

## 常用 Crate
- clap — CLI 参数解析
- serde / serde_yaml — 序列化
- chrono — 日期时间处理
- tokio — 异步运行时
- log / env_logger — 日志系统
- reqwest — HTTP 客户端
- rusqlite / sqlx — SQLite 数据库
- dioxus / dioxus-cli — 跨平台 UI 框架

## SQLite 数据库

Rust 生态中常用的 SQLite 库：

```bash
# rusqlite — 轻量同步库，基于官方 SQLite C API
cargo add rusqlite --features bundled

# sqlx — 异步库，支持编译时 SQL 检查（需 DATABASE_URL 或 offline mode）
cargo add sqlx --features sqlite,runtime-tokio
```

**rusqlite（同步）** — 与 sqlite3 使用体验一致，适合参数化查询场景：

```rust
use rusqlite::Connection;

let conn = Connection::open("data.db")?;
conn.execute(
    "CREATE TABLE IF NOT EXISTS entries (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )",
    [],
)?;
```

**sqlx（异步）** — 适合 tokio 异步环境，自带连接池：

```rust
use sqlx::SqlitePool;

let pool = SqlitePool::connect("sqlite:data.db?mode=rwc").await?;
let row = sqlx::query("SELECT name FROM entries WHERE id = ?")
    .bind(1)
    .fetch_one(&pool)
    .await?;
```

## Dioxus UI 框架

dioxus-cli v0.7.9 — Dioxus 官方 CLI 工具，用于创建、开发、打包 Dioxus 应用。

```bash
# 安装指定版本
cargo install dioxus-cli --version 0.7.9

# 创建新项目
dx new my-app

# 开发模式（Web 端热重载）
dx serve

# 构建生产版本
dx build --release

# 打包桌面应用（Windows/macOS/Linux）
dx bundle
```

Dioxus 0.7 关键特性：
- 类 React 的组件模型与 hooks
- 支持 Web、桌面端、移动端、TUI 多平台
- `RSX` 宏编写声明式 UI
- 内置状态管理（`use_signal`）与事件系统

**1. 事件处理绑定闭包**：

```rust
rsx! {
    button {
        onclick: move |_| log::info!("点击了按钮"),
        "点击"
    }
}
```

**2. 组件间通过 props 通信**：

```rust
#[component]
fn Counter(initial: i32) -> Element {
    let mut count = use_signal(|| initial);
    rsx! { "计数: {count}" }
}
```

### Hooks 规则（关键教训）

**绝对禁止在闭包内调用 hook**——包括 `use_context_provider(|| ...)`、`use_memo(|| ...)`、事件闭包等。Dioxus 0.7 会报 `BorrowMutError: The hook list is already borrowed`，页面 panic 无法渲染。

**错误写法**（闭包内调用 `use_signal`，运行时报错）：

```rust
#[component]
fn App() -> Element {
    use_context_provider(|| GlobalState {
        base: use_signal(|| "https://...".into()),  // ❌ 闭包内调 hook
    });
    // ...
}
```

**正确写法**（hook 全部在组件函数体顶层，闭包只做数据组装）：

```rust
#[component]
fn App() -> Element {
    let state = GlobalState {
        base: use_signal(|| "https://...".into()),   // ✅ 函数体顶层
    };
    use_context_provider(move || state);             // ✅ 闭包内无 hook
}
```

排查方法：运行时报错定位到 `dioxus-core/src/scope/_context.rs` 的 `BorrowMutError`，即 hook 在非顶层调用。

### RSX 循环与闭包借用（关键教训）

**1. RSX for 循环体内禁止写 `let` 语句**。Dioxus 0.7 的 RSX 宏不支持循环体内的局部声明，需先在组件函数体预构建数据：

```rust
// ❌ for 循环体内无法写 let
for item in items.iter() {
    let item = item.clone();   // 编译错误
    label { ... }
}

// ✅ 先在函数体构建 (Copy id, 标签) 元组
let item_pairs: Vec<(i64, String)> = items
    .iter()
    .map(|it| (it.id, item_label(it)))
    .collect();
for (item_id, item_text) in item_pairs.clone() {   // 按值迭代，闭包捕获 Copy 的 i64
    input { onchange: move |e| { let id = item_id; /* ... */ } }
}
```

**2. 循环内闭包捕获**：捕获集合元素（如 `Vec<Item>`）的引用会报 `borrowed value does not live long enough`。闭包需要 `'static`，应只捕获 Copy 类型（如 `i64` id）或先 clone 出 owned 数据。

**3. 元素 `type` 属性**：RSX 中 `type` 是 Rust 关键字，须写 `r#type`（如 `input { r#type: "text" }`、`r#type: "password"`、`r#type: "checkbox"`）。写 `input_type` 会报 `cannot find value input_type`。

### clippy 常见修复（Rust 1.96）

| 警告 | 修复 |
|---|---|
| `redundant_closure` | `use_signal(|| String::new())` → `use_signal(String::new)` |
| `needless_question_mark` | 去掉多余的 `Ok(...?)` 包裹 |
| `unnecessary_map_or` | `.map_or(true, \|f\| ...)` → `.is_none_or(\|f\| ...)` |
| `collapsible_if` | 嵌套 if 折叠为 `if let Some(x) = a && cond`（Rust 2024 let-chains） |
| `type_complexity` | 复杂元组类型抽成 `pub type Alias = ...` |
| `unnecessary_sort_by` | `sort_by(|a,b| key(a).cmp(&key(b)))` → `sort_by_key(|a| key(a))` |
