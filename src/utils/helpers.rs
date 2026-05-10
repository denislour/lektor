use crate::utils::Session;

pub fn tag_to_lang(tag: &str) -> &'static str {
    match tag {
        "rust" | "async" | "tokio" | "systems" | "lifetime" | "cli" => "rust",
        "python" | "typing" => "python",
        "go" | "golang" => "go",
        "database" | "postgres" | "sql" => "sql",
        "devops" | "k8s" | "docker" | "yaml" | "git" | "cron" => "yaml",
        "redis" | "kafka" | "prometheus" | "monitoring" => "bash",
        "api" | "architecture" | "testing" | "security" | "tools" | "editor" => "bash",
        _ => "plaintext",
    }
}

pub fn save_scroll_pos() {
    if let Some(y) = web_sys::window().and_then(|w| w.scroll_y().ok()) {
        Session::set_item("lektor_scroll", &y.to_string());
    }
}
