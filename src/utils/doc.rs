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
