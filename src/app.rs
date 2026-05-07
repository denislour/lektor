use crate::components::{Footer, Header};
use crate::data::load_posts;
use crate::i18n::Lang;
use crate::pages::{AboutPage, HomePage, PostPage};
use crate::stores::{AppCtx, AppStore, PostsStore, SearchStore};
use crate::utils::{Doc, Storage};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::params::Params;
use leptos_router::path;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, PartialEq, Eq, Params)]
pub struct PostParams {
    pub id: Option<String>,
}

#[component]
pub fn App() -> impl IntoView {
    let initial_theme = Storage::get_item("theme", "light");
    let initial_lang = match Storage::get_item("lang", "vi").as_str() {
        "en" => Lang::En,
        _ => Lang::Vi,
    };

    let ctx = AppCtx {
        app: AppStore::new(&initial_theme, initial_lang),
        search: SearchStore::new(),
        posts: PostsStore::new(load_posts()),
    };
    provide_context(ctx);

    Effect::new(move |_| {
        let q = ctx.search.query().get();
        ctx.posts.filter(&q);
    });

    Effect::new(move |_| {
        Doc::set_attr("data-theme", &ctx.app.theme().get());
    });

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let scroll_cb = wasm_bindgen::prelude::Closure::wrap(Box::new(move || {
            let scroll_y = web_sys::window().unwrap().scroll_y().unwrap_or(0.0);
            ctx.app.set_back_to_top(scroll_y > 300.0);
        }) as Box<dyn FnMut()>);
        let _ = window.set_onscroll(Some(scroll_cb.as_ref().unchecked_ref::<js_sys::Function>()));
        scroll_cb.forget();
    });

    let _ = js_sys::eval(
        "var hc=0;var ht=setInterval(function(){if(window.hljs){hljs.highlightAll();hc++;if(hc>4)clearInterval(ht)}},400)",
    );

    view! {
        <Router>
            <Header show_search=true />
            <main>
                <Routes fallback=|| view! { "404 Not Found" }>
                    <Route path=path!("") view=move || view! { <HomePage /> } />
                    <Route path=path!("about") view=AboutPage />
                    <Route path=path!("post") view=move || view! { <PostPage /> } />
                </Routes>
            </main>
            <Footer />
            <button
                class="
                    fixed bottom-8 right-8 z-100 w-9 h-9
                    rounded-md
                    bg-[var(--surface-card)]
                    border border-[var(--border-primary)]
                    text-[var(--text-secondary)] text-base
                    cursor-pointer
                    flex items-center justify-center
                    transition-all duration-150
                    font-mono
                    hover:border-[var(--brand-500)] hover:text-[var(--brand-500)]
                    hover:-translate-y-[2px]
                "
                class:opacity-0=move || !ctx.app.show_back_to_top().get()
                class:pointer-events-none=move || !ctx.app.show_back_to_top().get()
                class:opacity-100=move || ctx.app.show_back_to_top().get()
                class:pointer-events-auto=move || ctx.app.show_back_to_top().get()
                aria-label="Back to top"
                on:click=|_| {
                    if let Some(window) = web_sys::window() {
                        let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                }
            >
                "^"
            </button>
        </Router>
    }
}
