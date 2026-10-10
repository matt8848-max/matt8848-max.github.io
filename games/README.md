# games —— 游戏评价

一个游戏一个文件（`<id>.toml`），由 `webpage/build.rs` 在编译期汇总为 `src/data.rs` 中的 `GAMES`。
标签清单（8 个）**硬编码**在 `webpage/src/tags.rs` 的 `GAME_TAGS`，游戏文件里的 `tags` **必须**取自它。

字段（除 `name` 外均可省略）：

| 字段 | 类型 | 说明 |
|---|---|---|
| `name` | string | 游戏名称（必填） |
| `platform` | string | 平台，如 `PC`、`Nintendo Switch` |
| `status` | string | 游玩状态，如 `已通关`、`进行中` |
| `score` | number | 评分（10 分制） |
| `release_date` | string | 发行日期，ISO `YYYY-MM-DD`（如 `2000-09-28`） |
| `rating` | string | 评级，**文字形式**（如 `PEGI 12`、`ESRB T`），不使用评级标识图 |
| `tags` | string[] | 标签，**取值必须来自 `src/tags.rs` 的 `GAME_TAGS`** |
| `review` | string | 评价正文（可用 `"""..."""` 多行） |

> 更通俗的玩法分类（如 RTS、4X 大战略、战棋 SRPG）请写进 `review` 正文，而不是新增标签。

## 标签（硬编码于 `webpage/src/tags.rs`）

8 个合法标签**硬编码**在 `webpage/src/tags.rs` 的 `GAME_TAGS`（每项含英文原名 `name`、中文译名 `zh`、解释 `desc`）。**该分类已锁定，不再新增**：

```text
Games That Are Fun to Control
Adventure Games & Visual Novels
Cinematic & Story-Focused Games
Licensed Games
Games Based on Real Life
Building & Crafting Games
Rhythm Games
Sports Games
```

游戏文件里出现清单之外的标签时，`cargo test` 会**直接失败**——校验由 `src/tags.rs` 的
`test_game_tags_should_cover_all_games` 完成。

## 樱井政博对 8 个标签的讲解

以上 8 类出自樱井政博（桜井政博）关于「**游戏性之外的乐趣**」的分享：游戏除了以「博弈 / 风险与回报」为核心的"游戏性"之外，还存在许多同样能带来乐趣的要素，他举了 8 个代表性的例子，正好对应这 8 个标签。

- 视频（B04《游戏性：游戏性之外的乐趣》）：<https://www.bilibili.com/video/BV14v4y1D7pA>
- 中文演讲实录《聊一聊游戏的本质（下）》第 11 节「案例10：『游戏性』之外的乐趣」：<https://club.gamersky.com/activity/609004>

| 标签 | 樱井政博的观点（整理） |
|---|---|
| **Games That Are Fun to Control** | 单单"操控"电子游戏本身就让人觉得好玩——屏幕上那个从现实物理规律中解放出来的空间，对生活在现实中的我们来说不可思议且充满魅力。要做到"光是操作就很好玩"，需要合适的机制与对操作手感的敏锐调整。樱井的结论是："虽然没有（传统意义的）游戏性，但是好玩。" |
| **Adventure Games & Visual Novels** | 这类游戏里，"阅读故事的乐趣要优先于博弈"。它是小说和电影的延伸，但并不是故事有趣就算游戏——还要配合电脑特性（分支、演出等）来创作剧本，才会诞生游戏故事特有的趣味。例：《恐怖惊魂夜》；而《逆转裁判》则博弈成分更重，说明分界线往往是模糊的。 |
| **Cinematic & Story-Focused Games** | 影像（包括电影）和故事本身并"没有游戏性"，却已成为游戏性之外的重要精华：通过影像展示的角色与世界能加深玩家印象、提高沉浸感；有趣的故事会让人想知道后续，从而激发推进游戏的欲望。 |
| **Licensed Games** | 可看作"面向该作品粉丝的商品群"——比起"游戏是否有趣"，更重视能否让人享受该作品所表现的世界，因此非粉丝者的评价参考价值有限。但反过来，只要游戏本身足够有趣，也能吸引原作粉丝之外的玩家（例：《女神转生》多数玩家反而不知道其原作小说）。 |
| **Games Based on Real Life** | 以"一模一样再现现实世界"为目标存在表现极限，而由此产生的违和感本身也能生出乐趣；在模拟世界里做现实中做不到的事，是游戏性之外非常有趣的要素。例：电车模拟游戏——开门的一声"卡拉卡拉"音效，就让玩家感到现实世界的门被打开，通过模拟现实增加了信息量。 |
| **Building & Crafting Games** | 编辑系、手工系游戏，"只要在制作东西就能体验到乐趣"，即使与游戏性无关。若再把制作与"风险与回报"挂钩（为抵御敌人而做道具、为收集素材而冒险），有可能飞跃性地变得更有趣；与现实再现也相通（《动物之森》的家具比《我的世界》更贴近现实、更好理解）。 |
| **Rhythm Games** | 在讨论音乐游戏的"游戏性"之前要知道：合着节奏发出声音的乐趣，源自最原始的喜悦，人类可以纯粹地享受它；而且越是玩听熟了的曲子越能沉浸。在此基础上加入博弈与风险回报会更有挑战（例：《狂热节拍》系列——比完美时机稍晚即判定失败）。 |
| **Sports Games** | 真实的运动本身自带博弈，搬到电脑上时就已具备某种游戏性；由现实存在的队伍与选手构成，仅此就能让游戏好玩，再加上玩家能明确操控自己的队伍，便能沉浸在比赛的悲欢离合之中。 |

## 排序

游戏页按**发行日期**升序排列；发行日期相同则按**名称**升序排列。两者均为 Rust 字符串序
（日期以 ISO `YYYY-MM-DD` 存储，字符串序即时间先后）。排序逻辑见 `webpage/src/data.rs` 的 `filter_games`。

## 示例文件

```toml
name = "示例游戏"
platform = "PC"
status = "已通关"
score = 8.5
release_date = "2012-11-26"
rating = "PEGI 3"
tags = ["Games That Are Fun to Control", "Building & Crafting Games"]
review = """
一段评价……
"""
```

