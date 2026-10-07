# projects —— 自用项目

一个项目一个文件（`<id>.toml`），由 `webpage/build.rs` 在编译期汇总为 `src/data.rs` 中的 `PROJECTS`。

字段（除 `name` 外均可省略）：

| 字段 | 类型 | 说明 |
|---|---|---|
| `name` | string | 项目名称（必填） |
| `purpose` | string | 项目用途 / 简介 |
| `url` | string | 项目地址；**可为空**，为空时项目页以「地址不公开」占位 |

## 排序

项目页按**名称**升序排列（Rust 字符串序，即 UTF-8 字节序）。排序逻辑见 `webpage/src/data.rs` 的 `search_projects`。

## 示例文件

```toml
name = "示例项目"
purpose = "一句话说明这个项目是做什么的。"
url = "https://example.com/repo"
```
