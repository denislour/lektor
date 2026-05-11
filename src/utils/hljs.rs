use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub struct Hljs;

impl Hljs {
    fn call_hljs() {
        let h = match js_sys::Reflect::get(&js_sys::global(), &"hljs".into()) {
            Ok(h) if !h.is_undefined() => h,
            _ => return,
        };
        let f = match js_sys::Reflect::get(&h, &"highlightAll".into()) {
            Ok(f) => f,
            _ => return,
        };
        if let Some(func) = f.dyn_ref::<js_sys::Function>() {
            let _ = func.call0(&h);
        }
    }

    pub fn highlight() {
        if let Some(window) = web_sys::window() {
            let cb = Closure::wrap(Box::new(Self::call_hljs) as Box<dyn FnMut()>);
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                0,
            );
            cb.forget();
        }
    }

    pub fn init() {
        if let Some(window) = web_sys::window() {
            let cb = Closure::wrap(Box::new(Self::call_hljs) as Box<dyn FnMut()>);
            if let Ok(handle) = window.set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                400,
            ) {
                cb.forget();
                let w = window.clone();
                let clear = Closure::wrap(Box::new(move || {
                    let _ = w.clear_interval_with_handle(handle);
                }) as Box<dyn FnMut()>);
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    clear.as_ref().unchecked_ref(),
                    2000,
                );
                clear.forget();
            }
        }
    }
}
