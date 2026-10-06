//! 知识库：单输入框检索 + 书籍卡片网格。

use dioxus::prelude::*;

use crate::components::book_card::BookCard;
use crate::data::{search_books, BOOKS};
use crate::route::Route;

/// 知识库样式。
const KNOWLEDGE_CSS: Asset = asset!("/assets/knowledge.css");

/// 知识库组件。
///
/// 只提供一个按书名检索的搜索框：关键字为空时列出全部书籍，否则做模糊匹配。
#[component]
pub fn Knowledge() -> Element {
    // 检索关键字；空串表示列出全部书籍
    let mut query = use_signal(String::new);
    let books = search_books(BOOKS, &query());

    rsx! {
        document::Stylesheet { href: KNOWLEDGE_CSS }
        main { class: "kb",
            header { class: "kb-topbar",
                Link { class: "kb-back", to: Route::Home {}, "← 返回主页" }
                h1 { class: "kb-title", "知识库" }
            }
            input {
                class: "kb-search",
                r#type: "search",
                placeholder: "输入书名快速搜索…",
                value: query,
                oninput: move |event| query.set(event.value()),
            }
            if books.is_empty() {
                p { class: "kb-empty", "没有找到匹配的书籍。" }
            } else {
                div { class: "kb-grid",
                    for book in books {
                        BookCard { book: *book }
                    }
                }
            }
        }
    }
}
