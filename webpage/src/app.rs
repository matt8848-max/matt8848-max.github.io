//! 应用根组件。

use dioxus::prelude::*;

use crate::route::Route;

/// 站点图标。
const FAVICON: Asset = asset!("/assets/favicon.ico");

/// 全站基础样式：仅包含全局重置与主题变量，各页面的样式放在各自的独立 CSS 中。
const BASE_CSS: Asset = asset!("/assets/base.css");

/// 应用根组件：设置图标与基础样式，并挂载路由器。
#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: BASE_CSS }
        Router::<Route> {}
    }
}
