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

pub struct Storage;

impl Storage {
    pub fn get_item(key: &str, default: &str) -> String {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .and_then(|s| s.get_item(key).ok().flatten())
            .unwrap_or_else(|| default.to_string())
    }

    pub fn set_item(key: &str, value: &str) {
        if let Some(s) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            let _ = s.set_item(key, value);
        }
    }
}

pub struct Session;

impl Session {
    pub fn get_item(key: &str) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.session_storage().ok())
            .flatten()
            .and_then(|s| s.get_item(key).ok().flatten())
    }

    pub fn set_item(key: &str, value: &str) {
        if let Some(s) = web_sys::window()
            .and_then(|w| w.session_storage().ok())
            .flatten()
        {
            let _ = s.set_item(key, value);
        }
    }

    pub fn remove_item(key: &str) {
        if let Some(s) = web_sys::window()
            .and_then(|w| w.session_storage().ok())
            .flatten()
        {
            let _ = s.remove_item(key);
        }
    }
}

pub struct Doc;

impl Doc {
    pub fn set_attr(name: &str, value: &str) {
        let el = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element());
        if let Some(el) = el {
            let _ = el.set_attribute(name, value);
        }
    }
}
