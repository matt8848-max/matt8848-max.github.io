//! `build.rs` 使用的纯工具函数。
//!
//! 本模块只依赖标准库，便于同时被构建脚本（`build.rs`）与集成测试
//! （`tests/build_util.rs`）复用，从而让构建期的纯逻辑也能被 `cargo test` 覆盖。
//!
//! 这里的函数负责：生成稳定的 ASCII 标识、HTML 转义、拼接文章页模板，
//! 把「返回主页 / 返回知识库」浮动按钮注入任意 HTML 页面。

/// 注入到静态页右下角的浮动导航按钮组：`返回主页` + `返回知识库`。
///
/// 之所以放在**右下角**而非左上角：书籍静态页（尤其 mdbook 产物）左上角是侧边栏目录/
/// 菜单栏，浮动按钮会遮挡内容；右下角基本是空白页脚区，不会挡住正文。
///
/// 两个按钮均使用 `target="_top"`，以便在书籍被 `iframe` 嵌入时也能让最外层窗口跳转。
/// 内联样式避免为静态页额外引入样式表请求。返回知识库指向哈希路由
/// （`/#/knowledge`），整页回到站点根后由前端路由渲染知识库页。
pub fn nav_buttons_snippet(home_href: &str, knowledge_href: &str) -> String {
    format!(
        "<div style=\"position:fixed;right:16px;bottom:16px;z-index:2147483647;\
        display:flex;flex-direction:column;align-items:flex-end;gap:8px\">\
        <a href=\"{knowledge}\" target=\"_top\" style=\"{btn}\">← 返回知识库</a>\
        <a href=\"{home}\" target=\"_top\" style=\"{btn}\">← 返回主页</a></div>",
        btn = NAV_BUTTON_STYLE,
        knowledge = knowledge_href,
        home = home_href
    )
}

/// 浮动导航按钮的公共内联样式（靛蓝胶囊，与站点主题一致）。
const NAV_BUTTON_STYLE: &str = "display:inline-flex;align-items:center;gap:6px;\
    padding:8px 14px;border-radius:999px;background:#4f46e5;color:#fff;\
    font-family:system-ui,-apple-system,'Segoe UI','Microsoft YaHei',sans-serif;\
    font-size:14px;line-height:1;text-decoration:none;box-shadow:0 4px 14px rgba(0,0,0,.25)";

/// 把右下角导航按钮组注入到 HTML 页面。
///
/// 优先插入到 `</body>` 之前；若页面没有 `</body>`，则直接追加到末尾。
pub fn inject_nav_buttons(html: &str, home_href: &str, knowledge_href: &str) -> String {
    let snippet = nav_buttons_snippet(home_href, knowledge_href);
    match find_ignore_ascii_case(html, "</body>") {
        Some(index) => {
            let mut out = String::with_capacity(html.len() + snippet.len());
            out.push_str(&html[..index]);
            out.push_str(&snippet);
            out.push_str(&html[index..]);
            out
        }
        None => {
            let mut out = String::with_capacity(html.len() + snippet.len());
            out.push_str(html);
            out.push_str(&snippet);
            out
        }
    }
}

/// 转义 HTML 文本节点/属性中的特殊字符，避免标题等用户内容破坏页面结构。
pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// 由文件名/路径推导一个稳定的 ASCII 标识（URL 短名）。
///
/// 规则：保留 ASCII 字母与数字并统一小写，其余字符折叠为单个 `-`；
/// 若结果为空（例如纯中文文件名），则回退为基于内容哈希的 `book-<hash>`，
/// 以保证不同书籍之间不会互相覆盖，且每次构建结果一致。
pub fn ascii_id(raw: &str) -> String {
    let stem = std::path::Path::new(raw)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(raw);

    let mut slug = String::with_capacity(stem.len());
    let mut pending_dash = false;
    for ch in stem.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            pending_dash = false;
        } else if !pending_dash {
            // 连续的非法字符只折叠成一个分隔符，避免出现 `a--b`
            slug.push('-');
            pending_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        format!("book-{:016x}", fnv1a(stem))
    } else {
        trimmed.to_string()
    }
}

/// 生成单篇 Markdown 文章的完整 HTML 页面，并在末尾注入右下角导航按钮组。
///
/// `body_html` 应为已经转换好的正文 HTML 片段（不含 `<html>` 外壳）。
pub fn article_page(title: &str, body_html: &str, home_href: &str, knowledge_href: &str) -> String {
    let safe_title = escape_html(title);
    let mut out = String::with_capacity(body_html.len() + ARTICLE_CSS.len() + 256);
    out.push_str("<!DOCTYPE html>\n<html lang=\"zh-CN\">\n<head>\n");
    out.push_str("<meta charset=\"utf-8\">\n");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str("<title>");
    out.push_str(&safe_title);
    out.push_str("</title>\n<style>");
    out.push_str(ARTICLE_CSS);
    out.push_str("</style>\n</head>\n<body>\n<article class=\"dx-article\">\n");
    // Markdown 正文若自带一级标题，就不再重复渲染标题，避免页面上出现两个 `<h1>`
    if !body_html.contains("<h1") {
        out.push_str("<h1>");
        out.push_str(&safe_title);
        out.push_str("</h1>\n");
    }
    out.push_str(body_html);
    out.push_str("\n</article>\n</body>\n</html>\n");
    inject_nav_buttons(&out, home_href, knowledge_href)
}

/// 文章页内联样式：深色、居中、适合长文阅读。
const ARTICLE_CSS: &str = concat!(
    ":root{color-scheme:dark}",
    "body{margin:0;background:#0f1116;color:#e8eaf0;",
    "font-family:system-ui,-apple-system,'Segoe UI','Microsoft YaHei',sans-serif;line-height:1.7}",
    ".dx-article{max-width:820px;margin:0 auto;padding:64px 20px 96px}",
    ".dx-article h1{font-size:1.9rem;margin:0 0 1.5rem}",
    ".dx-article h2{font-size:1.4rem;margin:2rem 0 1rem;border-bottom:1px solid #2a2f3a;padding-bottom:.3rem}",
    ".dx-article h3{font-size:1.15rem;margin:1.6rem 0 .8rem}",
    ".dx-article a{color:#8b8cf0}",
    ".dx-article pre{background:#171a21;border:1px solid #2a2f3a;border-radius:12px;",
    "padding:14px 16px;overflow:auto}",
    ".dx-article code{font-family:ui-monospace,SFMono-Regular,Consolas,monospace;font-size:.92em}",
    ".dx-article :not(pre)>code{background:#171a21;border-radius:6px;padding:.15em .4em}",
    ".dx-article blockquote{margin:1rem 0;padding:.4rem 1rem;border-left:3px solid #4f46e5;",
    "color:#9aa3b2}",
    ".dx-article table{border-collapse:collapse}",
    ".dx-article th,.dx-article td{border:1px solid #2a2f3a;padding:.4rem .7rem}",
);

/// 在 `haystack` 中查找 `needle`（ASCII 大小写不敏感），返回字节下标。
///
/// 仅用于定位 `</body>` 这类 ASCII 标记，因此按字节比较是安全的。
fn find_ignore_ascii_case(haystack: &str, needle: &str) -> Option<usize> {
    let hay = haystack.as_bytes();
    let pat = needle.as_bytes();
    if pat.is_empty() || pat.len() > hay.len() {
        return None;
    }
    (0..=hay.len() - pat.len()).find(|&start| {
        hay[start..start + pat.len()]
            .iter()
            .zip(pat)
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
    })
}

/// FNV-1a 64 位哈希，用于为无语义可用的名称生成稳定后缀。
fn fnv1a(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
