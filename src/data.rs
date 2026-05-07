use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PostMeta {
    pub title: Option<String>,
    pub date: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "readTime")]
    pub read_time: Option<u32>,
    pub excerpt: Option<String>,
    #[serde(rename = "thumbnailCode")]
    pub thumbnail_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub read_time: u32,
    pub excerpt: String,
    pub thumbnail_code: String,
    pub content: String,
    pub html: String,
}

impl Post {
    fn from_md(id: &str, raw: &str) -> Option<Self> {
        let (meta_yaml, body) = Self::split_frontmatter(raw)?;
        let meta: PostMeta = serde_yaml::from_str(meta_yaml).ok()?;
        let html = render_md(body);

        Some(Post {
            id: id.to_string(),
            title: meta.title.unwrap_or_else(|| id.to_string()),
            date: meta.date.unwrap_or_else(|| "2026-01-01".into()),
            tags: meta.tags.unwrap_or_default(),
            read_time: meta.read_time.unwrap_or(5),

            excerpt: meta.excerpt.unwrap_or_default(),
            thumbnail_code: meta.thumbnail_code.unwrap_or_else(|| "// code".into()),
            content: body.to_lowercase(),
            html,
        })
    }

    fn split_frontmatter(raw: &str) -> Option<(&str, &str)> {
        let s = raw.trim();
        if !s.starts_with("---") {
            return None;
        }
        let end = s[3..].find("\n---")?;
        Some((&s[3..3 + end], &s[3 + end + 4..]))
    }
}

fn render_md(md: &str) -> String {
    let mut ext = comrak::options::Extension::default();
    ext.table = true;
    ext.autolink = true;
    ext.strikethrough = true;
    let mut render = comrak::options::Render::default();
    render.r#unsafe = true;
    let opts = comrak::Options {
        extension: ext,
        render,
        ..Default::default()
    };
    comrak::markdown_to_html(md, &opts)
}

pub fn format_date(date_str: &str) -> String {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        if let Ok(m) = parts[1].parse::<usize>() {
            if (1..=12).contains(&m) {
                return format!("{} {} {}", parts[2], months[m - 1], parts[0]);
            }
        }
    }
    date_str.to_string()
}

pub fn load_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    for (id, raw) in &crate::posts_list::ALL_POSTS {
        if let Some(post) = Post::from_md(id, raw) {
            posts.push(post);
        }
    }
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}
