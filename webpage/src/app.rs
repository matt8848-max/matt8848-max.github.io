//! 应用根组件。

use dioxus::prelude::*;

use crate::route::Route;

/// 站点图标：ICO 内含 16/32/48 三种尺寸；另备 PNG 供现代浏览器 / iOS / Android 取用。
const FAVICON: Asset = asset!("/assets/favicon.ico");

/// 32×32 PNG 标签页图标。
const FAVICON_PNG: Asset = asset!("/assets/favicon-32.png");

/// 180×180 PNG，iOS 添加到主屏时使用。
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon-180.png");

/// 192×192 PNG，Android / PWA 场景使用。
const ICON_192: Asset = asset!("/assets/icon-192.png");

/// 全站基础样式：仅包含全局重置与主题变量，各页面的样式放在各自的独立 CSS 中。
const BASE_CSS: Asset = asset!("/assets/base.css");

/// 应用根组件：设置图标与基础样式，并挂载路由器。
#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: FAVICON_PNG }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: APPLE_TOUCH_ICON }
        document::Link { rel: "icon", r#type: "image/png", sizes: "192x192", href: ICON_192 }
        document::Stylesheet { href: BASE_CSS }
        Router::<Route> {}
    }
}
