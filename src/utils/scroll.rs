use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub struct Scroll;

impl Scroll {
    pub fn watch(mut callback: Box<dyn FnMut(f64)>) -> Box<dyn FnOnce() + Send + Sync> {
        let Some(window) = web_sys::window() else { return Box::new(|| {}) };

        let w = window.clone();
        let cb = Closure::wrap(Box::new(move || {
            callback(w.scroll_y().unwrap_or(0.0));
        }) as Box<dyn FnMut()>);
        let _ = window.set_onscroll(Some(cb.as_ref().unchecked_ref::<js_sys::Function>()));
        cb.forget();

        let cleanup = window.clone();
        Box::new(move || {
            let _ = cleanup.set_onscroll(None);
        })
    }

    pub fn smooth_scroll_to(id: &str, delay_ms: i32) {
        let Some(window) = web_sys::window() else { return };
        let id = id.to_string();
        let cb = Closure::wrap(Box::new(move || {
            if let Some(el) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.get_element_by_id(&id))
            {
                let _ = el.scroll_into_view();
            }
        }) as Box<dyn FnMut()>);
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            delay_ms,
        );
        cb.forget();
    }
}
