//! 404 页面：提示地址不存在并提供返回主页入口。

use dioxus::prelude::*;

use crate::route::Route;

/// 404 页面样式。
const NOT_FOUND_CSS: Asset = asset!("/assets/not_found.css");

/// 404 组件。
///
/// `segments` 是未匹配到的路径片段，仅用于向用户展示其访问的地址。
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    rsx! {
        document::Stylesheet { href: NOT_FOUND_CSS }
        main { class: "nf",
            p { class: "nf-code", "404" }
            h1 { class: "nf-title", "页面不存在" }
            p { class: "nf-desc",
                "你访问的地址 "
                code { class: "nf-path", "{path}" }
                " 找不到对应的内容。"
            }
            Link { class: "nf-home", to: Route::Home {}, "返回主页" }
        }
    }
}
