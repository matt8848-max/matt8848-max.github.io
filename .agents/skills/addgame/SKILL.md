---
name: addgame
description: 向本仓库新增或修改一款游戏的评价条目（games/ 下的 *.toml）。当用户要求"添加/新增一个游戏评价"、编辑游戏信息、或涉及 games/ 数据时触发。说明字段格式、标签必须取自 webpage/src/tags.rs 硬编码的 8 个标签（该分类已锁定）、文件名即排序，以及构建 / 测试 / 本地验证流程。
---

# addgame —— 新增游戏评价

指导 AI 在本项目（个人静态主页，Rust + Dioxus）里新增或修改一款游戏的评价条目。

## 何时使用

- 用户要求"新增 / 添加一款游戏评价"、"把某款游戏加进游戏页"
- 用户要求修改 `games/*.toml` 里的游戏信息（评分、评价、标签等）

## 关键约束

1. **一个游戏一个文件**：`games/<id>.toml`，`<id>` 用 ASCII 短名（如 `stellaris.toml`）。
   **文件名即列表排序依据**（`build.rs` 按文件名字典序排列），要调整顺序就改文件名。
2. **`tags` 必须取自 `webpage/src/tags.rs` 中硬编码的 8 个标签**（`GAME_TAGS` 的 `name` 字段），禁止自造；一个游戏可带多个标签。
   该 8 分类**已锁定，不再新增**——更通俗的玩法分类（RTS、4X 大战略、战棋 SRPG 等）写进 `review` 正文。
   出现清单外的标签，`cargo test`（`test_game_tags_should_cover_all_games`）会**直接失败**。
3. 标签清单是代码常量（`webpage/src/tags.rs`），不再有 `tags.toml` 之类的外部清单文件。
4. 字段（除 `name` 外均可省略）：`name` / `platform` / `status` / `score` / `tags` / `review`。
   **没有 `played_at`**（游玩时间字段已移除，不要写）。

## 字段

| 字段 | 类型 | 说明 |
|---|---|---|
| `name` | string | 游戏名称（必填） |
| `platform` | string | 平台，如 `PC`、`Nintendo Switch` |
| `status` | string | 游玩状态，如 `已通关`、`进行中` |
| `score` | number | 评分（10 分制，可省略） |
| `tags` | string[] | 标签，**必须来自 `webpage/src/tags.rs` 的 `GAME_TAGS`** |
| `review` | string | 评价正文，多行用 `"""..."""` |

标签的含义见 `games/README.md`「樱井政博对 8 个标签的讲解」。

## 步骤

1. 读 `webpage/src/tags.rs` 的 `GAME_TAGS`，为这款游戏挑一个或多个合法标签。
   需要联网核实游戏信息（平台、发行、评级等）时，**本机只能通过 Bing 检索**（维基百科等无法访问）。
2. 在 `games/` 新建 `<id>.toml`（或改已有文件），按上表填写字段。
3. `cargo test --manifest-path webpage/Cargo.toml`：其中 `test_game_tags_should_cover_all_games` 会校验标签合法性。
4. `cargo clippy --manifest-path webpage/Cargo.toml -- -D warnings` 与 `cargo fmt --manifest-path webpage/Cargo.toml`。
5. 本地预览验证游戏页：`dx build --release --platform web` 后执行
   `uv run --no-project python -m http.server 8088 --directory webpage/target/dx/webpage/release/web/public`，
   打开 `http://localhost:8088/#/games` 检查卡片、搜索与标签筛选。

## 示例

`games/stellaris.toml`：

```toml
name = "群星"
platform = "PC"
status = "进行中"
score = 10
tags = ["Building & Crafting Games"]
review = """Paradox（P社）出品的太空 4X 大战略……"""
```

