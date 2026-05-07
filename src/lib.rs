use leptos::{mount::mount_to_body, prelude::view};
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod data;
mod i18n;
mod pages;
mod posts_list {
    include!(concat!(env!("OUT_DIR"), "/posts_list.rs"));
}
mod stores;
mod utils;
use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| {
        view! { <App /> }
    });
}
