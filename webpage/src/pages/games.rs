//! 游戏评价页：顶部搜索栏 + 左侧标签筛选 + 右侧评价列表（列表在自身区域内滚动）。

use dioxus::prelude::*;

use crate::data::{filter_games, Game, GAMES};
use crate::route::Route;
use crate::tags::GAME_TAGS;

/// 游戏评价页样式。
const GAMES_CSS: Asset = asset!("/assets/games.css");

/// 「?」浮层与视口底部的距离小于该像素时，改为向上弹出，避免被视口底缘裁掉。
const TIP_FLIP_THRESHOLD: f64 = 200.0;

/// 判断某标签当前是否处于勾选状态。
///
/// 抽成函数是为了在 RSX 的 `for` 循环体内直接调用：RSX 循环体不允许声明局部变量。
fn is_checked(checked: &[String], tag: &str) -> bool {
    checked.iter().any(|item| item == tag)
}

/// 拼接游戏条目的 meta 行文案：把非空的 `platform` / `status` 用 ` · ` 连接。
///
/// 两者都为空时返回 `None`，页面据此整行不渲染，避免出现空段或多余分隔符。
fn game_meta(game: &Game) -> Option<String> {
    let parts: Vec<&str> = [game.platform, game.status]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}

/// 取当前视口高度（px）。
///
/// 在非浏览器环境（如宿主机上的 `cargo test`）拿不到时返回 `0`，此时浮层按向下弹出处理。
fn viewport_height() -> f64 {
    web_sys::window()
        .and_then(|window| window.inner_height().ok())
        .and_then(|height| height.as_f64())
        .unwrap_or(0.0)
}

/// 游戏评价组件。
///
/// 数据来自 `games/` 下「一个游戏一个 TOML 文件」的编译期汇总；标签清单硬编码在
/// [`crate::tags::GAME_TAGS`]。默认勾选全部标签（即展示全部游戏）；取消勾选某标签，只会隐藏
/// 「其余标签也都未勾选」的游戏——游戏只要带任一已勾选标签就仍会显示（见 `filter_games`）。
/// 点击每个标签旁的「?」会浮出该标签的英文原名与解释。
#[component]
pub fn Games() -> Element {
    // 关键字；空串表示不按名称过滤
    let mut query = use_signal(String::new);
    // 已勾选的标签（存英文原名）；默认全部勾选（等价于不筛选）
    let mut checked_tags = use_signal(|| {
        GAME_TAGS
            .iter()
            .map(|tag| tag.name.to_string())
            .collect::<Vec<String>>()
    });
    // 当前展开解释浮层的标签（英文原名）；None 表示不显示浮层
    let mut open_tip = use_signal(|| Option::<&'static str>::None);
    // 当前浮层是否向上弹出（点击点靠近视口底部时为 true）
    let mut tip_upward = use_signal(|| false);

    // 读取一次勾选状态，供渲染与筛选共用，避免重复借用信号
    let checked = checked_tags();
    let games = filter_games(GAMES, &query(), &checked);

    rsx! {
        document::Stylesheet { href: GAMES_CSS }
        div {
            class: "games-page",
            // 点击页面任意处（含左右留白）收起解释浮层；点「?」或浮层内部会阻止冒泡，不受影响
            onclick: move |_| open_tip.set(None),
            main { class: "games",
                header { class: "games-topbar",
                    Link { class: "games-back", to: Route::Home {}, "← 返回主页" }
                    h1 { class: "games-title", "游戏评价" }
                }
                input {
                    class: "games-search",
                    r#type: "search",
                    placeholder: "输入游戏名快速搜索…",
                    value: query,
                    oninput: move |event| query.set(event.value()),
                }
                div { class: "games-body",
                    aside { class: "games-filter",
                        span { class: "games-filter-label", "标签筛选" }
                        for tag in GAME_TAGS {
                            div { class: "games-tag-row",
                                label { class: "games-tag",
                                    input {
                                        r#type: "checkbox",
                                        checked: is_checked(&checked, tag.name),
                                        onchange: move |event| {
                                            let mut list = checked_tags();
                                            if event.checked() {
                                                // 防止重复勾选
                                                if !list.iter().any(|item| item.as_str() == tag.name) {
                                                    list.push(tag.name.to_string());
                                                }
                                            } else {
                                                list.retain(|item| item.as_str() != tag.name);
                                            }
                                            checked_tags.set(list);
                                        },
                                    }
                                    span { class: "games-tag-name", "{tag.zh}" }
                                }
                                button {
                                    class: "games-tag-help",
                                    r#type: "button",
                                    title: "查看英文原名与说明",
                                    onclick: move |event| {
                                        // 阻止冒泡，避免被外层「点击别处关闭」立刻收起
                                        event.stop_propagation();
                                        if open_tip() == Some(tag.name) {
                                            // 再点一次收起
                                            open_tip.set(None);
                                        } else {
                                            // 点击点离视口底部不足阈值时改为向上弹，避免被底缘裁掉
                                            let click_y = event.client_coordinates().y;
                                            tip_upward
                                                .set(viewport_height() - click_y < TIP_FLIP_THRESHOLD);
                                            open_tip.set(Some(tag.name));
                                        }
                                    },
                                    "?"
                                }
                                if open_tip() == Some(tag.name) {
                                    div {
                                        class: if tip_upward() { "games-tag-tip games-tag-tip-up" } else { "games-tag-tip" },
                                        // 点浮层内部（如选中文字）不关闭
                                        onclick: move |event| event.stop_propagation(),
                                        strong { class: "games-tag-tip-name", "{tag.name}" }
                                        p { class: "games-tag-tip-desc", "{tag.desc}" }
                                    }
                                }
                            }
                        }
                    }
                    section { class: "games-results",
                        if games.is_empty() {
                            p { class: "games-empty", "没有找到匹配的游戏。" }
                        } else {
                            ul { class: "games-list",
                                for game in games {
                                    li { class: "games-item",
                                        div { class: "games-head",
                                            span { class: "games-name", "{game.name}" }
                                            if let Some(score) = game.score {
                                                span { class: "games-score", "{score}" }
                                            }
                                        }
                                        if let Some(meta) = game_meta(game) {
                                            p { class: "games-meta", "{meta}" }
                                        }
                                        if !game.tags.is_empty() {
                                            div { class: "games-badges",
                                                for tag in game.tags {
                                                    span { class: "games-badge", "{tag}" }
                                                }
                                            }
                                        }
                                        p { class: "games-review", "{game.review}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 构造一条只看 platform / status 的测试游戏，其余字段留空。
    fn game_with(platform: &'static str, status: &'static str) -> Game {
        Game {
            name: "测试游戏",
            platform,
            status,
            score: None,
            tags: &[],
            review: "",
        }
    }

    /// 两个字段都非空时，应以 ` · ` 连接。
    #[test]
    fn test_game_meta_should_join_present_fields() {
        assert_eq!(
            game_meta(&game_with("PC", "已通关")).as_deref(),
            Some("PC · 已通关")
        );
    }

    /// 缺失的字段应被跳过，不产生多余分隔符。
    #[test]
    fn test_game_meta_should_skip_empty_fields() {
        assert_eq!(game_meta(&game_with("PC", "")).as_deref(), Some("PC"));
        assert_eq!(
            game_meta(&game_with("", "已通关")).as_deref(),
            Some("已通关")
        );
    }

    /// 两个字段都为空时应返回 `None`，让页面整行不渲染。
    #[test]
    fn test_game_meta_should_be_none_when_all_empty() {
        assert_eq!(game_meta(&game_with("", "")), None);
    }
}
