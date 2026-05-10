use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

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
