//! `build/util.rs` 中纯函数的集成测试。
//!
//! 构建脚本中的逻辑无法被 `cargo test` 直接执行，这里通过 `#[path]` 复用同一份源码，
//! 让清单解析之外的纯逻辑（标识生成、HTML 注入、转义、页面模板）也获得单元测试覆盖。

#[path = "../build/util.rs"]
mod util;

/// 验证 ASCII 别名生成：常规名称、含非 ASCII 字符的名称、以及连续分隔符折叠。
#[test]
fn test_ascii_id_should_slugify_ascii_and_strip_non_ascii() {
    assert_eq!(util::ascii_id("Algorithm"), "algorithm");
    assert_eq!(util::ascii_id("My-Book Name.md"), "my-book-name");
    // 中文等非 ASCII 字符应被丢弃，两侧 ASCII 片段以单个 `-` 连接
    assert_eq!(
        util::ascii_id("WSL2_Ubuntu安装CUDA、torch.md"),
        "wsl2-ubuntu-cuda-torch"
    );
}

/// 验证连续非法字符只折叠成一个分隔符，避免出现 `a--b` 这类不稳定别名。
///
/// 注意 `ascii_id` 以路径的文件名主体（file_stem）为输入，因此这里用文件名而非路径验证。
#[test]
fn test_ascii_id_should_collapse_repeated_separators() {
    assert_eq!(util::ascii_id("a--b.md"), "a-b");
    assert_eq!(util::ascii_id("---hello---"), "hello");
}

/// 验证纯非 ASCII 名称会回退为稳定的、彼此不同的 `book-<hash>` 别名。
#[test]
fn test_ascii_id_should_fallback_for_non_ascii_names() {
    let first = util::ascii_id("算法.md");
    let second = util::ascii_id("算法.md");
    let other = util::ascii_id("数据结构.md");

    assert!(first.starts_with("book-"), "别名应以 book- 开头：{first}");
    assert_eq!(first.len(), "book-".len() + 16, "应带 16 位十六进制哈希");
    assert_eq!(first, second, "同一输入必须稳定");
    // 不同名称必须产生不同别名，否则会互相覆盖
    assert_ne!(first, other);
}

/// 验证 HTML 转义覆盖全部特殊字符，确保用户内容不会破坏页面结构。
#[test]
fn test_escape_html_should_escape_special_characters() {
    assert_eq!(
        util::escape_html(r#"<a href="x">&'</a>"#),
        "&lt;a href=&quot;x&quot;&gt;&amp;&#39;&lt;/a&gt;"
    );
    // 普通文本（含中文）保持不变
    assert_eq!(util::escape_html("算法笔记"), "算法笔记");
}

/// 验证右下角导航按钮组固定在右下角（避开书籍正文/侧边栏），且同时含两个入口。
#[test]
fn test_nav_buttons_should_be_positioned_bottom_right() {
    let snippet = util::nav_buttons_snippet("/", "/#/knowledge");

    assert!(snippet.contains("position:fixed"), "应为浮动定位");
    assert!(snippet.contains("right:16px"), "应贴右边");
    assert!(snippet.contains("bottom:16px"), "应贴底部");
    assert!(snippet.contains("返回主页"), "应包含返回主页入口");
    assert!(snippet.contains("返回知识库"), "应包含返回知识库入口");
    assert!(
        snippet.contains(r#"href="/#/knowledge""#),
        "知识库入口应指向哈希路由"
    );
}

/// 验证导航按钮组优先注入到 `</body>` 之前，且大小写不敏感。
#[test]
fn test_inject_nav_buttons_should_insert_before_body_end() {
    let html = "<html><body><h1>hi</h1></body></html>";
    let injected = util::inject_nav_buttons(html, "/", "/#/knowledge");

    let button = injected.find("返回主页").expect("应包含返回按钮");
    let body_end = injected.find("</body>").expect("应保留 </body>");
    assert!(button < body_end, "按钮应出现在 </body> 之前");
    assert!(injected.contains(r#"href="/""#), "按钮应指向主页");
    assert!(
        injected.contains(r#"href="/#/knowledge""#),
        "按钮应指向知识库"
    );

    // 大写闭合标签同样能被定位
    let upper = util::inject_nav_buttons("<BODY>x</BODY>", "/", "/#/knowledge");
    assert!(upper.find("返回主页").unwrap() < upper.find("</BODY>").unwrap());
}

/// 验证没有 `</body>` 的片段会退化为直接追加，保证按钮始终存在。
#[test]
fn test_inject_nav_buttons_should_append_when_body_missing() {
    let injected = util::inject_nav_buttons("<div>片段</div>", "/home", "/#/knowledge");
    assert!(injected.starts_with("<div>片段</div>"));
    assert!(injected.contains("返回主页"));
    assert!(injected.contains(r#"href="/home""#));
}

/// 验证文章页模板包含转义后的标题、正文以及注入的导航按钮；
/// 正文没有一级标题时由模板补上。
#[test]
fn test_article_page_should_render_title_body_and_button() {
    let page = util::article_page("<标题>", "<p>正文</p>", "/", "/#/knowledge");

    assert!(page.contains("<title>&lt;标题&gt;</title>"), "标题需转义");
    assert!(
        page.contains("<h1>&lt;标题&gt;</h1>"),
        "无一级标题时应由模板补上"
    );
    assert!(page.contains("<p>正文</p>"), "正文需原样嵌入");
    assert!(page.contains("返回主页"), "应注入返回主页按钮");
    assert!(page.contains("返回知识库"), "应注入返回知识库按钮");
    assert!(page.contains("</body>"), "应生成完整 HTML");
}

/// 验证正文自带一级标题时不会再重复渲染模板标题，避免出现两个 `<h1>`。
#[test]
fn test_article_page_should_not_duplicate_existing_h1() {
    let page = util::article_page("标题", "<h1>正文标题</h1>", "/", "/#/knowledge");

    assert_eq!(page.matches("<h1>").count(), 1, "只应保留正文自带的标题");
    assert!(page.contains("<title>标题</title>"));
}
