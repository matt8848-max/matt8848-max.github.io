# games —— 游戏评价

一个游戏一个文件（`<id>.toml`），由 `webpage/build.rs` 在编译期汇总为 `src/data.rs` 中的 `GAMES`。

字段（除 `name` 外均可省略）：

| 字段 | 类型 | 说明 |
|---|---|---|
| `name` | string | 游戏名称（必填） |
| `platform` | string | 平台，如 `PC`、`Switch` |
| `status` | string | 游玩状态，如 `已通关`、`进行中` |
| `score` | number | 评分（10 分制） |
| `played_at` | string | 游玩时间，如 `2026-01` |
| `tags` | string[] | 标签 |
| `review` | string | 评价正文（可用 `"""..."""` 多行） |

示例（文件名即排序依据，当前目录为空，页面显示「暂无评价」）：

```toml
name = "示例游戏"
platform = "PC"
status = "已通关"
score = 8.5
played_at = "2026-01"
tags = ["Roguelike", "独立"]
review = """
一段评价……
"""
```
