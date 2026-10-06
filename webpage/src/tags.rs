//! 游戏标签清单（硬编码）。
//!
//! 8 个标签取自樱井政博关于「游戏性之外的乐趣」的分享，是游戏页筛选复选框与
//! 「?」解释浮层的唯一数据源，同时也是校验 `games/*.toml` 标签合法性的依据。
//! 各标签含义的详细说明见 `games/README.md`「樱井政博对 8 个标签的讲解」。

/// 一个游戏标签。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GameTag {
    /// 英文原名；也是 `games/*.toml` 里 `tags` 的取值、筛选用的键
    pub name: &'static str,
    /// 页面展示的中文译名
    pub zh: &'static str,
    /// 「?」浮动窗口里显示的说明
    pub desc: &'static str,
}

/// 全部合法游戏标签（8 类，已锁定，不再新增）。
pub const GAME_TAGS: &[GameTag] = &[
    GameTag {
        name: "Games That Are Fun to Control",
        zh: "操控手感出色的游戏",
        desc: "单单「操控」电子游戏本身就让人觉得好玩；要做到「光是操作就很好玩」，需要合适的机制与对操作手感的细致调整。",
    },
    GameTag {
        name: "Adventure Games & Visual Novels",
        zh: "冒险游戏与视觉小说",
        desc: "阅读故事的乐趣优先于博弈，是小说与电影的延伸；但要配合电脑特性（分支、演出）来创作，才会诞生游戏故事特有的趣味。",
    },
    GameTag {
        name: "Cinematic & Story-Focused Games",
        zh: "影像与剧情向游戏",
        desc: "影像与故事本身没有游戏性，却能加深对角色的印象、提升沉浸感，并用悬念激发推进欲，是游戏性之外的重要精华。",
    },
    GameTag {
        name: "Licensed Games",
        zh: "授权改编游戏",
        desc: "面向原作粉丝的商品群，比起「游戏是否有趣」更看重能否享受原作世界；非粉丝者的评价参考价值有限。",
    },
    GameTag {
        name: "Games Based on Real Life",
        zh: "拟真现实题材游戏",
        desc: "以再现现实为目标存在表现极限，而由此产生的违和感也能生出乐趣；在模拟世界里做现实中做不到的事尤其有趣。",
    },
    GameTag {
        name: "Building & Crafting Games",
        zh: "建造与制作游戏",
        desc: "只要在制作东西就能获得乐趣，即使与游戏性无关；若再把制作与风险回报挂钩（为御敌而造物、为素材而冒险）会更有趣。",
    },
    GameTag {
        name: "Rhythm Games",
        zh: "音乐节奏游戏",
        desc: "合着节奏发出声音源自最原始的喜悦，可以纯粹地享受，越是听熟的曲子越沉浸；再叠加博弈与风险回报会更具挑战。",
    },
    GameTag {
        name: "Sports Games",
        zh: "体育游戏",
        desc: "真实运动自带博弈，由现实中的队伍与选手构成，加上玩家能明确操控自己的队伍，便能沉浸于比赛的悲欢离合。",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::GAMES;

    /// 标签数量与唯一性：恰好 8 个，且英文原名两两互不重复。
    #[test]
    fn test_game_tags_should_be_unique() {
        assert_eq!(GAME_TAGS.len(), 8, "标签分类应锁定为 8 个");
        for (i, tag) in GAME_TAGS.iter().enumerate() {
            let duplicated = GAME_TAGS
                .iter()
                .skip(i + 1)
                .any(|other| other.name == tag.name);
            assert!(!duplicated, "标签「{}」重复登记", tag.name);
        }
    }

    /// 每个游戏文件里出现的标签都必须来自本清单。
    ///
    /// 这一步替代了原先在 `build.rs` 里输出 `cargo:warning` 的构建期校验：
    /// 若某个 `games/<id>.toml` 写了清单外的标签，`cargo test` 会直接失败。
    #[test]
    fn test_game_tags_should_cover_all_games() {
        for game in GAMES {
            for tag in game.tags {
                assert!(
                    GAME_TAGS.iter().any(|known| known.name == *tag),
                    "游戏「{}」使用了未登记的标签：{}",
                    game.name,
                    tag
                );
            }
        }
    }

    /// 不变量：每个游戏都至少带一个标签。
    ///
    /// 理论上不应存在无标签的游戏；一旦 `games/` 下出现，这里会失败，从而在测试阶段发现它。
    #[test]
    fn test_games_should_all_have_at_least_one_tag() {
        for game in GAMES {
            assert!(
                !game.tags.is_empty(),
                "游戏「{}」没有任何标签；理论上不应存在无标签游戏",
                game.name
            );
        }
    }
}
