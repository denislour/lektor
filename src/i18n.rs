#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Vi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I18nKey {
    NavHome,
    NavPosts,
    NavAbout,
    NavSearchPrompt,
    BackToHomepage,
    OpenMenu,
    CloseMenu,
    HeroBlogLabel,
    HeroFnWriteIdeas,
    HeroArrowString,
    HeroDescription,
    HeroPostsLatest,
    HeroAbout,
    FeaturedBadge,
    MinRead,
    ReadMoreArrow,
    PostsSectionTitle,
    PostsSectionDesc,
    PostCardMinRead,
    PostCardRead,
    PaginationPrev,
    PaginationNext,
    SearchResultsLabel,
    SearchNoResults,
    PostBackToHome,
    PostNotFound,
    AboutPara1,
    AboutPara2,
    AboutItem1,
    AboutItem2,
    AboutItem3,
    AboutItem4,
    AboutItem5,
    AboutBlockquote,
    AboutPara3,
    AboutSignoff,
    FooterText,
}

pub fn tr(lang: Lang, key: I18nKey) -> &'static str {
    match (lang, key) {
        (Lang::En, I18nKey::NavHome) => "~/home",
        (Lang::Vi, I18nKey::NavHome) => "~/home",
        (Lang::En, I18nKey::NavPosts) => "~/posts",
        (Lang::Vi, I18nKey::NavPosts) => "~/posts",
        (Lang::En, I18nKey::NavAbout) => "~/about",
        (Lang::Vi, I18nKey::NavAbout) => "~/about",
        (Lang::En, I18nKey::NavSearchPrompt) => "~/search:",
        (Lang::Vi, I18nKey::NavSearchPrompt) => "~/search:",
        (Lang::En, I18nKey::BackToHomepage) => "Back to homepage",
        (Lang::Vi, I18nKey::BackToHomepage) => "Về trang chủ",
        (Lang::En, I18nKey::OpenMenu) => "Open menu",
        (Lang::Vi, I18nKey::OpenMenu) => "Mở menu",
        (Lang::En, I18nKey::CloseMenu) => "Close menu",
        (Lang::Vi, I18nKey::CloseMenu) => "Đóng menu",
        (Lang::En, I18nKey::HeroBlogLabel) => "λ blog",
        (Lang::Vi, I18nKey::HeroBlogLabel) => "λ blog",
        (Lang::En, I18nKey::HeroFnWriteIdeas) => "fn write_ideas",
        (Lang::Vi, I18nKey::HeroFnWriteIdeas) => "fn write_ideas",
        (Lang::En, I18nKey::HeroArrowString) => "-> String",
        (Lang::Vi, I18nKey::HeroArrowString) => "-> String",
        (Lang::En, I18nKey::HeroDescription) => {
            "Just a corner to write about backend, Rust, Python, and system architecture — Coffee not included."
        }
        (Lang::Vi, I18nKey::HeroDescription) => {
            "Một góc nhỏ để viết linh tinh về backend, Rust, Python, với kiến trúc hệ thống."
        }
        (Lang::En, I18nKey::HeroPostsLatest) => "~/posts/latest",
        (Lang::Vi, I18nKey::HeroPostsLatest) => "~/posts/latest",
        (Lang::En, I18nKey::HeroAbout) => "~/about",
        (Lang::Vi, I18nKey::HeroAbout) => "~/about",
        (Lang::En, I18nKey::FeaturedBadge) => "featured",
        (Lang::Vi, I18nKey::FeaturedBadge) => "nổi bật",
        (Lang::En, I18nKey::MinRead) => "min read",
        (Lang::Vi, I18nKey::MinRead) => "phút đọc",
        (Lang::En, I18nKey::ReadMoreArrow) => "read more ->",
        (Lang::Vi, I18nKey::ReadMoreArrow) => "đọc thêm →",
        (Lang::En, I18nKey::PostsSectionTitle) => "~/posts",
        (Lang::Vi, I18nKey::PostsSectionTitle) => "~/posts",
        (Lang::En, I18nKey::PostsSectionDesc) => "Recent writings on backend & systems",
        (Lang::Vi, I18nKey::PostsSectionDesc) => "Bài viết gần đây về backend & systems",
        (Lang::En, I18nKey::PostCardMinRead) => "min read",
        (Lang::Vi, I18nKey::PostCardMinRead) => "phút đọc",
        (Lang::En, I18nKey::PostCardRead) => "read ->",
        (Lang::Vi, I18nKey::PostCardRead) => "đọc →",
        (Lang::En, I18nKey::PaginationPrev) => "<- prev",
        (Lang::Vi, I18nKey::PaginationPrev) => "← trước",
        (Lang::En, I18nKey::PaginationNext) => "next ->",
        (Lang::Vi, I18nKey::PaginationNext) => "tiếp →",
        (Lang::En, I18nKey::SearchResultsLabel) => "results",
        (Lang::Vi, I18nKey::SearchResultsLabel) => "kết quả",
        (Lang::En, I18nKey::SearchNoResults) => "No posts found",
        (Lang::Vi, I18nKey::SearchNoResults) => "Không tìm thấy bài viết",
        (Lang::En, I18nKey::PostBackToHome) => "<- back to home",
        (Lang::Vi, I18nKey::PostBackToHome) => "← về trang chủ",
        (Lang::En, I18nKey::PostNotFound) => "Post not found",
        (Lang::Vi, I18nKey::PostNotFound) => "Không tìm thấy bài viết",
        (Lang::En, I18nKey::AboutPara1) => {
            "Hi, I'm Lektor - a backend engineer working with Rust, Python, and distributed systems."
        }
        (Lang::Vi, I18nKey::AboutPara1) => {
            "Chào, mình là Lektor - một thằng backend engineer làm việc với Rust, Python và distributed systems."
        }
        (Lang::En, I18nKey::AboutPara2) => {
            "This blog is where I write about things I encounter at work:"
        }
        (Lang::Vi, I18nKey::AboutPara2) => {
            "Cái blog này là chỗ mình viết linh tinh về mấy thứ mình gặp trong công việc:"
        }
        (Lang::En, I18nKey::AboutItem1) => {
            "Bugs I don't understand why they run (and why they don't)"
        }
        (Lang::Vi, I18nKey::AboutItem1) => {
            "Mấy cái bug không hiểu tại sao nó chạy (và tại sao nó không chạy)"
        }
        (Lang::En, I18nKey::AboutItem2) => {
            "System architecture - from a tiny VPS to distributed systems"
        }
        (Lang::Vi, I18nKey::AboutItem2) => {
            "Kiến trúc hệ thống - từ con VPS con con cho đến distributed systems"
        }
        (Lang::En, I18nKey::AboutItem3) => {
            "Rust - the language I love most, even though it hurts my brain with the borrow checker"
        }
        (Lang::Vi, I18nKey::AboutItem3) => {
            "Rust - ngôn ngữ mình yêu nhất, dù nó hay làm mình đau đầu với cái borrow checker"
        }
        (Lang::En, I18nKey::AboutItem4) => {
            "Database, DevOps, monitoring - things every backend engineer deals with"
        }
        (Lang::Vi, I18nKey::AboutItem4) => {
            "Database, DevOps, monitoring - mấy thứ mà backend nào cũng phải đụng vào"
        }
        (Lang::En, I18nKey::AboutItem5) => {
            "Go - when I want goroutines instead of async/await headaches"
        }
        (Lang::Vi, I18nKey::AboutItem5) => {
            "Go - when I want goroutines instead of async/await headaches"
        }
        (Lang::En, I18nKey::AboutBlockquote) => {
            "Write to understand yourself better. Share to know you're not alone in the bug pile."
        }
        (Lang::Vi, I18nKey::AboutBlockquote) => {
            "Viết để hiểu mình hơn. Share để biết mình không đơn độc trong đống bug."
        }
        (Lang::En, I18nKey::AboutPara3) => {
            "I'm not an expert, nor a professional blogger. I just write to remember, to share, and sometimes to vent after a long debug session."
        }
        (Lang::Vi, I18nKey::AboutPara3) => {
            "Mình không phải expert, cũng không phải blogger chuyên nghiệp. Mình chỉ viết để nhớ, để share, và thỉnh thoảng để xả stress sau những buổi debug dài."
        }
        (Lang::En, I18nKey::AboutSignoff) => "- Lektor, writing at 2am",
        (Lang::Vi, I18nKey::AboutSignoff) => "- Lektor, viết lúc 2h sáng",
        (Lang::En, I18nKey::FooterText) => "print(\"Lektor\")",
        (Lang::Vi, I18nKey::FooterText) => "print(\"Lektor\")",
    }
}
