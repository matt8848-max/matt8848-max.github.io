//! 构建脚本：把仓库根下 `books/`、`mdbooks/`、`games/` 的内容汇总成前端可用形式。
//!
//! 职责：
//! 1. 读取 `books/manifest.toml`、`mdbooks/manifest.toml` 与 `games/*.toml`；
//! 2. 单篇 Markdown 文章 → 生成静态 HTML（含「返回主页」浮动按钮）到 `webpage/public/books/`；
//! 3. mdbook 书籍 → 调用 `mdbook build --dest-dir` 输出到 `webpage/public/books/<id>/`，
//!    并给每个生成的 HTML 注入「返回主页」浮动按钮；
//! 4. 生成 `OUT_DIR/books.rs` 与 `OUT_DIR/games.rs`，供 `src/data.rs` 通过 `include!` 引入。
//!
//! `webpage/public/` 会被 dx 按 `Dioxus.toml` 的 `application.public_dir` 整棵复制进站点根，
//! 且不做文件名 hash —— 这正是 mdbook 内部相对链接能够保持有效的关键。

#[path = "build/util.rs"]
mod util;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

/// 「返回主页」按钮指向的地址（本站为 GitHub Pages 用户根站点）。
const HOME_HREF: &str = "/";

/// 书籍清单中的一个条目（`books/manifest.toml` 与 `mdbooks/manifest.toml` 的 `[[book]]`）。
#[derive(Debug, Deserialize)]
struct BookEntry {
    /// 展示名称
    name: String,
    /// 相对清单所在目录的路径（Markdown 文件或 mdbook 工程目录）
    path: String,
    /// 简介
    intro: String,
    /// 可选的 URL 短名（ASCII）；缺省时由 `path` 推导
    #[serde(default)]
    id: Option<String>,
}

/// 书籍清单文件。
#[derive(Debug, Default, Deserialize)]
struct Manifest {
    #[serde(default)]
    book: Vec<BookEntry>,
}

/// `games/` 下的单个游戏评价文件（一个游戏一个文件）。
#[derive(Debug, Deserialize)]
struct GameEntry {
    /// 游戏名称
    name: String,
    #[serde(default)]
    platform: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    score: Option<f64>,
    #[serde(default)]
    played_at: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    review: String,
}

/// 汇总后用于生成数据模块的书籍条目。
struct GenBook {
    name: String,
    intro: String,
    url: String,
    kind: &'static str,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    // 仓库根目录：webpage/ 的上一级
    let repo_root = manifest_dir
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.clone());

    let books_dir = repo_root.join("books");
    let mdbooks_dir = repo_root.join("mdbooks");
    let games_dir = repo_root.join("games");

    // 输入变化时重新运行本脚本
    for dir in [&books_dir, &mdbooks_dir, &games_dir] {
        println!("cargo:rerun-if-changed={}", dir.display());
    }

    let public_books = manifest_dir.join("public").join("books");
    // 清掉上一次的生成结果，避免已删除的书籍在产物里残留
    let _ = std::fs::remove_dir_all(&public_books);

    let mut books = Vec::new();
    books.extend(build_articles(&books_dir, &public_books));
    books.extend(build_mdbooks(&mdbooks_dir, &public_books));

    let games = load_games(&games_dir);

    write_books_module(&out_dir, &books);
    write_games_module(&out_dir, &games);
}

/// 构建 `books/` 下的单篇 Markdown 文章。
fn build_articles(books_dir: &Path, public_books: &Path) -> Vec<GenBook> {
    let manifest = load_manifest(&books_dir.join("manifest.toml"));
    let mut generated = Vec::with_capacity(manifest.book.len());

    for entry in manifest.book {
        let md_path = books_dir.join(&entry.path);
        let id = book_id(&entry);
        let markdown = match std::fs::read_to_string(&md_path) {
            Ok(text) => text,
            Err(err) => {
                warn(&format!("读取 {} 失败：{err}", md_path.display()));
                continue;
            }
        };

        let body = markdown_to_html(&markdown);
        let html = util::article_page(&entry.name, &body, HOME_HREF);
        let out_file = public_books.join(format!("{id}.html"));
        match write_file(&out_file, &html) {
            Ok(()) => generated.push(GenBook {
                name: entry.name,
                intro: entry.intro,
                url: format!("/books/{id}.html"),
                kind: "Article",
            }),
            Err(err) => warn(&format!("写入 {} 失败：{err}", out_file.display())),
        }
    }

    generated
}

/// 构建 `mdbooks/` 下的 mdbook 工程，并注入「返回主页」按钮。
fn build_mdbooks(mdbooks_dir: &Path, public_books: &Path) -> Vec<GenBook> {
    let manifest = load_manifest(&mdbooks_dir.join("manifest.toml"));
    let mut generated = Vec::with_capacity(manifest.book.len());

    for entry in manifest.book {
        let book_dir = mdbooks_dir.join(&entry.path);
        let id = book_id(&entry);
        let dest = public_books.join(&id);
        match run_mdbook(&book_dir, &dest) {
            Ok(()) => {
                inject_home_buttons(&dest, HOME_HREF);
                generated.push(GenBook {
                    name: entry.name,
                    intro: entry.intro,
                    url: format!("/books/{id}/index.html"),
                    kind: "MdBook",
                });
            }
            Err(err) => warn(&format!("构建 mdbook {} 失败：{err}", book_dir.display())),
        }
    }

    generated
}

/// 调用外部 `mdbook` 命令把 `book_dir` 构建到 `dest`。
///
/// `mdbook` 需在 PATH 中可用：本地已安装，CI 由 `taiki-e/install-action` 安装。
fn run_mdbook(book_dir: &Path, dest: &Path) -> Result<(), String> {
    if !book_dir.join("book.toml").is_file() {
        return Err(format!("{} 下缺少 book.toml", book_dir.display()));
    }
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }

    let output = Command::new("mdbook")
        .arg("build")
        .arg(book_dir)
        .arg("--dest-dir")
        .arg(dest)
        .output()
        .map_err(|err| format!("无法执行 mdbook（{err}）"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// 递归给目录下所有 HTML 注入「返回主页」按钮。
fn inject_home_buttons(dir: &Path, home_href: &str) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            inject_home_buttons(&path, home_href);
        } else if is_html(&path) {
            inject_home_button_file(&path, home_href);
        }
    }
}

/// 判断路径是否为 `.html` 文件。
fn is_html(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("html"))
}

/// 给单个 HTML 文件注入按钮（内容无变化时不写回，减少无谓 IO）。
fn inject_home_button_file(path: &Path, home_href: &str) {
    let Ok(html) = std::fs::read_to_string(path) else {
        return;
    };
    let injected = util::inject_home_button(&html, home_href);
    if injected != html {
        if let Err(err) = std::fs::write(path, injected) {
            warn(&format!("注入返回按钮到 {} 失败：{err}", path.display()));
        }
    }
}

/// 读取并解析书籍清单；文件缺失或解析失败时返回空清单（仅告警，不中断构建）。
fn load_manifest(path: &Path) -> Manifest {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Manifest::default();
    };
    match toml::from_str::<Manifest>(&text) {
        Ok(manifest) => manifest,
        Err(err) => {
            warn(&format!("解析 {} 失败：{err}", path.display()));
            Manifest::default()
        }
    }
}

/// 读取 `games/` 下所有 `.toml` 并解析为游戏条目（按文件名排序，保证顺序稳定）。
fn load_games(games_dir: &Path) -> Vec<GameEntry> {
    let mut games = Vec::new();
    let Ok(entries) = std::fs::read_dir(games_dir) else {
        return games;
    };

    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
        })
        .collect();
    paths.sort();

    for path in paths {
        match std::fs::read_to_string(&path) {
            Ok(text) => match toml::from_str::<GameEntry>(&text) {
                Ok(game) => games.push(game),
                Err(err) => warn(&format!("解析 {} 失败：{err}", path.display())),
            },
            Err(err) => warn(&format!("读取 {} 失败：{err}", path.display())),
        }
    }

    games
}

/// 把 Markdown 转换为 HTML 片段（启用表格、删除线、任务列表）。
fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut out = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut out, parser);
    out
}

/// 计算书籍的 URL 短名：优先使用清单里的 `id`，否则由 `path` 推导。
fn book_id(entry: &BookEntry) -> String {
    match entry.id.as_deref() {
        Some(id) if !id.is_empty() => id.to_string(),
        _ => util::ascii_id(&entry.path),
    }
}

/// 生成 `OUT_DIR/books.rs`。
fn write_books_module(out_dir: &Path, books: &[GenBook]) {
    let mut src = String::new();
    src.push_str("// @generated by build.rs：由 books/ 与 mdbooks/ 清单汇总，请勿手动修改。\n\n");
    src.push_str(BOOKS_PRELUDE);
    src.push_str("/// 全部书籍（编译期汇总）。\npub const BOOKS: &[Book] = &[\n");
    for book in books {
        src.push_str(&format!(
            "    Book {{ name: {:?}, intro: {:?}, url: {:?}, kind: BookKind::{} }},\n",
            book.name, book.intro, book.url, book.kind
        ));
    }
    src.push_str("];\n");

    let path = out_dir.join("books.rs");
    write_file(&path, &src).unwrap_or_else(|err| panic!("写入 {} 失败：{err}", path.display()));
}

/// 生成 `OUT_DIR/games.rs`。
fn write_games_module(out_dir: &Path, games: &[GameEntry]) {
    let mut src = String::new();
    src.push_str("// @generated by build.rs：由 games/*.toml 汇总，请勿手动修改。\n\n");
    src.push_str(GAMES_PRELUDE);
    src.push_str("/// 全部游戏评价（编译期汇总）。\npub const GAMES: &[Game] = &[\n");
    for game in games {
        let score = match game.score {
            Some(value) => format!("Some({value}f32)"),
            None => "None".to_string(),
        };
        let tags = game
            .tags
            .iter()
            .map(|tag| format!("{tag:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        src.push_str(&format!(
            "    Game {{ name: {:?}, platform: {:?}, status: {:?}, score: {score}, \
             played_at: {:?}, tags: &[{tags}], review: {:?} }},\n",
            game.name, game.platform, game.status, game.played_at, game.review
        ));
    }
    src.push_str("];\n");

    let path = out_dir.join("games.rs");
    write_file(&path, &src).unwrap_or_else(|err| panic!("写入 {} 失败：{err}", path.display()));
}

/// 写文件（自动创建父目录）。
fn write_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, contents)
}

/// 以 `cargo:warning` 形式输出告警，避免直接用打印替代日志。
fn warn(message: &str) {
    println!("cargo:warning={message}");
}

/// `books.rs` 中公开类型的前置声明。
const BOOKS_PRELUDE: &str = concat!(
    "/// 书籍类型。\n",
    "#[derive(Clone, Copy, PartialEq, Eq, Debug)]\n",
    "pub enum BookKind {\n",
    "    /// `books/` 下的单篇 Markdown 文章\n",
    "    Article,\n",
    "    /// `mdbooks/` 下的 mdbook 多页书籍\n",
    "    MdBook,\n",
    "}\n\n",
    "/// 一本可被知识库检索的书籍。\n",
    "#[derive(Clone, Copy, PartialEq, Eq, Debug)]\n",
    "pub struct Book {\n",
    "    /// 展示名称\n",
    "    pub name: &'static str,\n",
    "    /// 简介\n",
    "    pub intro: &'static str,\n",
    "    /// 相对站点根的静态页地址\n",
    "    pub url: &'static str,\n",
    "    /// 书籍类型\n",
    "    pub kind: BookKind,\n",
    "}\n\n",
);

/// `games.rs` 中公开类型的前置声明。
const GAMES_PRELUDE: &str = concat!(
    "/// 一条游戏评价。\n",
    "#[derive(Clone, Copy, PartialEq, Debug)]\n",
    "pub struct Game {\n",
    "    /// 游戏名称\n",
    "    pub name: &'static str,\n",
    "    /// 平台\n",
    "    pub platform: &'static str,\n",
    "    /// 游玩状态\n",
    "    pub status: &'static str,\n",
    "    /// 评分（10 分制）\n",
    "    pub score: Option<f32>,\n",
    "    /// 游玩时间（如 `2026-01`）\n",
    "    pub played_at: &'static str,\n",
    "    /// 标签\n",
    "    pub tags: &'static [&'static str],\n",
    "    /// 评价正文\n",
    "    pub review: &'static str,\n",
    "}\n\n",
);
