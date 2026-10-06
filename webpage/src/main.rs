//! 个人主页前端入口。
//!
//! 使用哈希路由（`HashHistory`）：GitHub Pages 不做 SPA 回退，
//! 哈希路由可避免直接刷新子路径时 404，并让未知路由稳定落到站点自己的 404 页面。

mod app;
mod components;
mod data;
mod pages;
mod route;
mod tags;

use std::rc::Rc;

/// 站点启动入口。
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::web::Config::new().history(Rc::new(dioxus::web::HashHistory::new(false))))
        .launch(app::App);
}
