//! 项目页：单输入框检索 + 项目列表。

use dioxus::prelude::*;

use crate::data::{search_projects, PROJECTS};
use crate::route::Route;

/// 项目页样式。
const PROJECTS_CSS: Asset = asset!("/assets/projects.css");

/// 项目组件。
///
/// 只提供一个按项目名检索的搜索框：关键字为空时列出全部项目，否则做模糊匹配。
/// 列表按项目名排序（见 `search_projects`）；地址为空的项目以「地址不公开」占位。
#[component]
pub fn Projects() -> Element {
    // 检索关键字；空串表示列出全部项目
    let mut query = use_signal(String::new);
    let projects = search_projects(PROJECTS, &query());

    rsx! {
        document::Stylesheet { href: PROJECTS_CSS }
        main { class: "projects",
            header { class: "projects-topbar",
                Link { class: "projects-back", to: Route::Home {}, "← 返回主页" }
                h1 { class: "projects-title", "项目" }
            }
            input {
                class: "projects-search",
                r#type: "search",
                placeholder: "输入项目名快速搜索…",
                value: query,
                oninput: move |event| query.set(event.value()),
            }
            if projects.is_empty() {
                p { class: "projects-empty", "没有找到匹配的项目。" }
            } else {
                ul { class: "projects-list",
                    for project in projects {
                        li { class: "projects-item",
                            div { class: "projects-name", "{project.name}" }
                            if !project.purpose.is_empty() {
                                p { class: "projects-purpose", "{project.purpose}" }
                            }
                            // 地址为空表示不公开，以灰色占位替代链接
                            if project.url.is_empty() {
                                span { class: "projects-url projects-url-empty", "地址不公开" }
                            } else {
                                a {
                                    class: "projects-url",
                                    href: "{project.url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "{project.url}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
