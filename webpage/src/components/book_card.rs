//! 书籍卡片：书本图案 + 标题 + 简介，整体是一个可点击的大按钮。

use dioxus::prelude::*;

use crate::data::Book;

/// 书籍卡片组件。
///
/// 点击后跳转到该书对应的静态页（Markdown 文章页或 mdbook 首页）。
/// 书本图案使用文字符号而非图片，符合「尽量少用图片」的要求。
#[component]
pub fn BookCard(book: Book) -> Element {
    rsx! {
        a { class: "book-card", href: "{book.url}",
            div { class: "book-card-icon", "aria-hidden": "true", "📖" }
            div { class: "book-card-title", "{book.name}" }
            div { class: "book-card-intro", "{book.intro}" }
        }
    }
}
