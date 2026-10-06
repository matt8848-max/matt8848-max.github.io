//! 书籍卡片：书本图案 + 标题 + 简介，整体是一个可点击的大按钮。

use dioxus::prelude::*;

use crate::data::{Book, BookKind};

/// 书籍卡片组件。
///
/// 点击后跳转到该书对应的静态页（Markdown 文章页或 mdbook 首页）。
/// 书本图案使用文字符号而非图片，符合「尽量少用图片」的要求；
/// 并按书籍类型区分图标（文章 📄 / mdbook 📘），方便一眼辨认。
#[component]
pub fn BookCard(book: Book) -> Element {
    // 不同来源的书籍用不同图标：`books/` 的单篇文章与 `mdbooks/` 的多页书籍
    let icon = match book.kind {
        BookKind::Article => "📄",
        BookKind::MdBook => "📘",
    };
    rsx! {
        a { class: "book-card", href: "{book.url}",
            div { class: "book-card-icon", "aria-hidden": "true", "{icon}" }
            div { class: "book-card-title", "{book.name}" }
            div { class: "book-card-intro", "{book.intro}" }
        }
    }
}
