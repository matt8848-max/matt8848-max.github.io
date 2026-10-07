//! 站点路由定义。

use dioxus::prelude::*;

use crate::pages::{
    games::Games, home::Home, knowledge::Knowledge, not_found::NotFound, projects::Projects,
};

/// 站点全部路由。
///
/// `/:..segments` 作为兜底，任何未匹配的地址都会渲染 [`NotFound`] 页面。
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    /// 主页：标题 + 一排排页面入口按钮
    #[route("/")]
    Home {},
    /// 知识库：检索并浏览书籍
    #[route("/knowledge")]
    Knowledge {},
    /// 游戏评价
    #[route("/games")]
    Games {},
    /// 项目：检索并浏览自用项目
    #[route("/projects")]
    Projects {},
    /// 未匹配的路径：404 页面
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
