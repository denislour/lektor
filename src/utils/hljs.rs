use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

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
