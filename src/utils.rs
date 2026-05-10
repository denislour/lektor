use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

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

pub struct Hljs;

impl Hljs {
    pub fn init() {
        if let Some(window) = web_sys::window() {
            let cb = Closure::wrap(Box::new(move || {
                if let Ok(hljs) = js_sys::Reflect::get(&js_sys::global(), &"hljs".into()) {
                    if !hljs.is_undefined() {
                        if let Ok(f) = hljs.dyn_into::<js_sys::Function>() {
                            let _ = f.call0(&js_sys::global());
                        }
                    }
                }
            }) as Box<dyn FnMut()>);

            if let Ok(handle) = window.set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                400,
            ) {
                cb.forget();

                let w = window.clone();
                let clear_cb = Closure::wrap(Box::new(move || {
                    let _ = w.clear_interval_with_handle(handle);
                }) as Box<dyn FnMut()>);
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    clear_cb.as_ref().unchecked_ref(),
                    2000,
                );
                clear_cb.forget();
            }
        }
    }
}

pub struct Scroll;

impl Scroll {
    pub fn watch(mut callback: Box<dyn FnMut(f64)>) -> Box<dyn FnOnce() + Send + Sync> {
        if let Some(window) = web_sys::window() {
            let w = window.clone();
            let cb = Closure::wrap(Box::new(move || {
                let scroll_y = w.scroll_y().unwrap_or(0.0);
                callback(scroll_y);
            }) as Box<dyn FnMut()>);
            let _ = window.set_onscroll(Some(cb.as_ref().unchecked_ref::<js_sys::Function>()));
            cb.forget();

            let w2 = window.clone();
            Box::new(move || {
                let _ = w2.set_onscroll(None);
            })
        } else {
            Box::new(|| {})
        }
    }
}
