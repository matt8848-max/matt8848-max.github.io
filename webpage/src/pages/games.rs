//! 游戏评价页：目前暂无数据，展示占位提示。

use dioxus::prelude::*;

use crate::data::GAMES;
use crate::route::Route;

/// 游戏评价页样式。
const GAMES_CSS: Asset = asset!("/assets/games.css");

/// 游戏评价组件。
///
/// 数据来自 `games/` 下「一个游戏一个 TOML 文件」的编译期汇总；当前为空。
#[component]
pub fn Games() -> Element {
    rsx! {
        document::Stylesheet { href: GAMES_CSS }
        main { class: "games",
            header { class: "games-topbar",
                Link { class: "games-back", to: Route::Home {}, "← 返回主页" }
                h1 { class: "games-title", "游戏评价" }
            }
            if GAMES.is_empty() {
                p { class: "games-empty", "暂无评价。" }
            } else {
                ul { class: "games-list",
                    for game in GAMES {
                        li { class: "games-item",
                            div { class: "games-head",
                                span { class: "games-name", "{game.name}" }
                                if let Some(score) = game.score {
                                    span { class: "games-score", "{score}" }
                                }
                            }
                            p { class: "games-meta", "{game.platform} · {game.status} · {game.played_at}" }
                            p { class: "games-review", "{game.review}" }
                        }
                    }
                }
            }
        }
    }
}
