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
