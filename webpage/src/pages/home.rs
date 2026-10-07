//! 主页：站点标题 + 一排排的页面入口按钮。

use dioxus::prelude::*;

use crate::route::Route;

/// 主页样式。
const HOME_CSS: Asset = asset!("/assets/home.css");

/// 主页组件。
///
/// 每个入口是一整行的大按钮；新增页面时在 `home-nav` 中追加一个 [`Link`] 即可。
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: HOME_CSS }
        main { class: "home",
            h1 { class: "home-title", "Persona" }
            p { class: "home-subtitle", "个人主页 · 学习与记录" }
            nav { class: "home-nav",
                Link { class: "home-button", to: Route::Knowledge {}, "知识库" }
                Link { class: "home-button", to: Route::Games {}, "游戏评价" }
                Link { class: "home-button", to: Route::Projects {}, "项目" }
            }
        }
    }
}
