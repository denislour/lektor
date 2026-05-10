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
