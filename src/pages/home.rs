use crate::components::{FeaturedPost, HeroSection, Pagination, PostCard};
use crate::data::Post;
use crate::i18n::*;
use crate::stores::AppCtx;
use crate::utils::Session;
use leptos::prelude::*;

const PER_PAGE: usize = 6;

#[component]
pub fn HomePage() -> impl IntoView {
    let AppCtx { app, search, posts } = use_context().expect("AppCtx not provided");
    let lang = app.lang();

    Effect::new(move |_| {
        let p = app.page().get();
        Session::set_item("lektor_page", &p.to_string());
    });

    Effect::new(move |_| {
        let _ = app.page().get();
        if let Some(y) = Session::get_item("lektor_scroll").and_then(|y| y.parse::<f64>().ok()) {
            Session::remove_item("lektor_scroll");
            let _ = js_sys::eval(&format!(
                "setTimeout(function(){{window.scrollTo(0,{})}},100)",
                y
            ));
        }
    });

    Effect::new(move |_| {
        let _ = app.page().get();
        let _ = search.is_searching().get();
        let _ = js_sys::eval(
            "requestAnimationFrame(function(){setTimeout(function(){if(window.hljs)window.hljs.highlightAll()},100)})",
        );
    });

    let grid_items = move || {
        let items = posts.items().get();
        match !search.is_searching().get() {
            true => items.into_iter().skip(1).collect(),
            false => items,
        }
    };

    let total_pages = move || {
        let count = grid_items().len();
        match count == 0 {
            true => 1,
            false => (count + PER_PAGE - 1) / PER_PAGE,
        }
    };

    let paginated = move || {
        let p = app.page().get();
        let items = grid_items();
        let start = (p - 1) * PER_PAGE;
        items
            .into_iter()
            .skip(start)
            .take(PER_PAGE)
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            <HeroSection />
            {move || {
                if !search.is_searching().get() {
                    view! { <FeaturedPost /> }.into_any()
                } else {
                    ().into_any()
                }
            }}
            <section class="py-16 reveal" id="posts-section">
                <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4">
                    <div class="text-center mb-10">
                        <h2 class="font-mono text-2xl font-semibold mb-1">{move || tr(lang.get(), I18nKey::PostsSectionTitle)}</h2>
                        <p class="text-[var(--text-secondary)] text-sm font-mono">{move || tr(lang.get(), I18nKey::PostsSectionDesc)}</p>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        {move || {
                            let items = paginated();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center py-12 font-mono text-sm text-[var(--text-secondary)] col-span-2">{move || tr(lang.get(), I18nKey::SearchNoResults)}</p>
                                }.into_any()
                            } else {
                                view! {
                                    <For each=move || paginated() key=|p: &Post| p.id.clone() children={move |p: Post| view! { <PostCard post=p /> }} />
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
            </section>
            <Pagination page=app.page() on_page_change=move |n| app.set_page(n) total=total_pages />
        </div>
    }
}
