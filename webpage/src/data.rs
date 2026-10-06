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
}
