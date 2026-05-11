use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub struct Hljs;

impl Hljs {
    pub fn highlight() {
        if let Ok(h) = js_sys::Reflect::get(&js_sys::global(), &"hljs".into()) {
            if !h.is_undefined() {
                if let Ok(f) = js_sys::Reflect::get(&h, &"highlightAll".into()) {
                    if let Some(func) = f.dyn_ref::<js_sys::Function>() {
                        let _ = func.call0(&h);
                    }
                }
            }
        }
    }

    pub fn init() {
        if let Some(window) = web_sys::window() {
            let cb = Closure::wrap(Box::new(move || {
                if let Ok(hljs) = js_sys::Reflect::get(&js_sys::global(), &"hljs".into()) {
                    if !hljs.is_undefined() {
                        if let Ok(f) = js_sys::Reflect::get(&hljs, &"highlightAll".into()) {
                            if let Some(func) = f.dyn_ref::<js_sys::Function>() {
                                let _ = func.call0(&hljs);
                            }
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
