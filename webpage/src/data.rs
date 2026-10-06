//! 编译期生成的数据（书籍、游戏）与其读取/检索逻辑。
//!
//! 数据本身由 `build.rs` 写入 `OUT_DIR`，这里通过 `include!` 引入，
//! 使页面在编译期即拿到全部静态内容，运行时无需额外请求。

include!(concat!(env!("OUT_DIR"), "/books.rs"));
include!(concat!(env!("OUT_DIR"), "/games.rs"));

/// 按书名检索书籍。
///
/// 关键字为空（或仅含空白）时返回全部书籍；否则做**大小写不敏感**的子串匹配。
/// 返回借用自入参的引用，避免拷贝书籍数据。
pub fn search_books<'a>(books: &'a [Book], query: &str) -> Vec<&'a Book> {
    let needle = query.trim().to_lowercase();
    books
        .iter()
        .filter(|book| needle.is_empty() || book.name.to_lowercase().contains(&needle))
        .collect()
}

/// 按关键字 + 勾选的标签筛选游戏。
///
/// - `query` 为空（或仅含空白）时不按关键字过滤；否则对游戏名做**大小写不敏感**子串匹配。
/// - `checked` 是当前处于勾选状态的标签集合（默认全选）：一个有标签的游戏被展示，
///   **只要它的任一标签在 `checked` 中**（OR 语义）——因此取消勾选某标签，只会隐藏
///   「其余标签也都未勾选」的游戏。每个游戏都至少带一个标签（见 `crate::tags` 的不变量
///   测试 `test_games_should_all_have_at_least_one_tag`），故这里无需为「无标签游戏」兜底。
///
/// 返回借用自入参的引用，避免拷贝游戏数据。
pub fn filter_games<'a>(games: &'a [Game], query: &str, checked: &[String]) -> Vec<&'a Game> {
    let needle = query.trim().to_lowercase();
    games
        .iter()
        .filter(|game| needle.is_empty() || game.name.to_lowercase().contains(&needle))
        .filter(|game| {
            game.tags
                .iter()
                .any(|tag| checked.iter().any(|candidate| candidate.as_str() == *tag))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 构造用于测试的书籍集合（与真实清单解耦，保证测试稳定）。
    fn sample_books() -> Vec<Book> {
        vec![
            Book {
                name: "Rust 程序设计",
                intro: "",
                url: "",
                kind: BookKind::Article,
            },
            Book {
                name: "算法",
                intro: "",
                url: "",
                kind: BookKind::MdBook,
            },
            Book {
                name: "Rust 实战",
                intro: "",
                url: "",
                kind: BookKind::Article,
            },
        ]
    }

    /// 验证空关键字会返回全部书籍（知识库默认列出全部）。
    #[test]
    fn test_search_books_should_return_all_when_query_empty() {
        let books = sample_books();
        assert_eq!(search_books(&books, "").len(), books.len());
    }

    /// 验证仅含空白的关键字同样被视作空。
    #[test]
    fn test_search_books_should_treat_whitespace_as_empty() {
        let books = sample_books();
        assert_eq!(search_books(&books, "   ").len(), books.len());
    }

    /// 验证匹配为大小写不敏感的子串匹配，并已忽略首尾空白。
    #[test]
    fn test_search_books_should_match_case_insensitively() {
        let books = sample_books();
        let hit = search_books(&books, "  RUST ");
        assert_eq!(hit.len(), 2);
        assert!(hit.iter().all(|book| book.name.contains("Rust")));
    }

    /// 验证无匹配时返回空集合（知识库展示空态提示）。
    #[test]
    fn test_search_books_should_return_empty_for_no_match() {
        let books = sample_books();
        assert!(search_books(&books, "python").is_empty());
    }

    /// 构造用于测试的游戏集合（标签取自真实分类，便于验证筛选语义）。
    fn sample_games() -> Vec<Game> {
        vec![
            Game {
                name: "Celeste",
                platform: "PC",
                status: "已通关",
                score: Some(9.0),
                tags: &["Games That Are Fun to Control"],
                review: "",
            },
            Game {
                name: "Rhythm Heaven",
                platform: "DS",
                status: "进行中",
                score: None,
                tags: &["Rhythm Games"],
                review: "",
            },
            Game {
                name: "混合类型",
                platform: "PC",
                status: "已通关",
                score: None,
                tags: &["Rhythm Games", "Sports Games"],
                review: "",
            },
        ]
    }

    /// 全部标签都勾选时，应返回全部游戏（默认全开 = 展示全部）。
    #[test]
    fn test_filter_games_should_return_all_when_all_tags_checked() {
        let games = sample_games();
        let checked: Vec<String> = vec![
            "Games That Are Fun to Control".to_string(),
            "Rhythm Games".to_string(),
            "Sports Games".to_string(),
        ];
        assert_eq!(filter_games(&games, "", &checked).len(), games.len());
    }

    /// 只要游戏「任一」标签被勾选即可见：只勾 Sports Games 时，
    /// 带 Rhythm + Sports 的「混合类型」也应显示（OR 语义）。
    #[test]
    fn test_filter_games_should_keep_game_when_any_tag_checked() {
        let games = sample_games();
        let checked = vec!["Sports Games".to_string()];
        let hits = filter_games(&games, "", &checked);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "混合类型");
    }

    /// 游戏的所有标签都未被勾选时，才应被过滤掉。
    #[test]
    fn test_filter_games_should_exclude_games_with_unchecked_tag() {
        let games = sample_games();
        let checked = vec!["Games That Are Fun to Control".to_string()];
        let hits = filter_games(&games, "", &checked);
        // 只剩 Celeste：其余游戏的标签（Rhythm / Sports）都未被勾选
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "Celeste");
    }

    /// 关键字匹配应大小写不敏感并忽略首尾空白。
    #[test]
    fn test_filter_games_should_match_query_case_insensitively() {
        let games = sample_games();
        // 全部标签勾选，仅检验关键字匹配
        let checked: Vec<String> = vec![
            "Games That Are Fun to Control".to_string(),
            "Rhythm Games".to_string(),
            "Sports Games".to_string(),
        ];
        let hits = filter_games(&games, "  CELESTE ", &checked);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "Celeste");
    }

    /// 关键字与标签筛选同时生效（交集）：名称命中且标签被勾选才可见。
    #[test]
    fn test_filter_games_should_combine_query_and_tag() {
        let games = sample_games();
        // 只勾选 Rhythm Games 且名称含 "rhythm"：只剩 Rhythm Heaven
        //（「混合类型」虽带 Rhythm Games 标签，但名称不含 "rhythm"）
        let checked = vec!["Rhythm Games".to_string()];
        let hits = filter_games(&games, "rhythm", &checked);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].name, "Rhythm Heaven");
    }

    /// 无匹配时应返回空集合（游戏页展示空态提示）。
    #[test]
    fn test_filter_games_should_return_empty_for_no_match() {
        let games = sample_games();
        let checked = vec!["Rhythm Games".to_string()];
        assert!(filter_games(&games, "zelda", &checked).is_empty());
    }
}
