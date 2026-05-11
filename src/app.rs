use crate::components::{Footer, Header};
use crate::data::load_posts;
use crate::i18n::Lang;
use crate::pages::{AboutPage, HomePage, PostPage};
use crate::stores::{AppCtx, AppStore, PostsStore, SearchStore};
use crate::utils::{Doc, Hljs, Scroll, Storage};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::params::Params;
use leptos_router::path;

#[derive(Debug, Clone, PartialEq, Eq, Params)]
pub struct PostParams {
    pub id: Option<String>,
}

#[allow(non_snake_case)]
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
        Doc::set_attr("data-theme", &ctx.app.theme().get());
    });

    Effect::new(move |_| {
        let app = ctx.app;
        let cleanup = Scroll::watch(Box::new(move |y| app.set_back_to_top(y > 300.0)));
        on_cleanup(cleanup);
    });

    Hljs::init();

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
