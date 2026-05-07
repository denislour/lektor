use crate::app::PostParams;

use crate::i18n::*;
use crate::stores::AppCtx;

use leptos::prelude::*;
use leptos_router::hooks::use_query;
use wasm_bindgen::JsCast;

#[component]
pub fn PostPage() -> impl IntoView {
    let AppCtx { app, posts, .. } = expect_context();
    let lang = app.lang();
    let query = use_query::<PostParams>();

    let post = move || {
        let id = query.get().ok().and_then(|p| p.id).unwrap_or_default();
        posts.items().get().into_iter().find(|p| p.id == id)
    };

    Effect::new(move |_| {
        let _ = post();
        if let Some(w) = web_sys::window() {
            let cb = wasm_bindgen::prelude::Closure::once(move || {
                let _ = js_sys::eval("if(window.hljs)hljs.highlightAll()");
            });
            let _ = w.set_timeout_with_callback(cb.as_ref().unchecked_ref::<js_sys::Function>());
            cb.forget();
        }
    });

    view! {
        <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4">
            <div class="mb-8">
                <a href="/" class="font-mono text-sm text-[var(--text-secondary)] hover:text-[var(--brand-500)]">
                    {move || tr(lang.get(), I18nKey::PostBackToHome)}
                </a>
            </div>
            {move || match post() {
                Some(p) => view! {
                    <article class="max-w-[720px] mx-auto font-body text-[var(--text-primary)] leading-relaxed">
                        <header class="mb-10 pb-8 border-b border-[var(--border-subtle)]">
                            <h1 class="font-mono text-3xl max-sm:text-2xl font-bold leading-tight mb-4">{p.title.clone()}</h1>
                            <div class="font-mono text-xs text-[var(--text-tertiary)] flex items-center gap-2 flex-wrap mb-4">
                                <time datetime={p.date.clone()}>{crate::data::format_date(&p.date)}</time>
                                <span>" " {p.read_time} " " {move || tr(lang.get(), I18nKey::MinRead)}</span>
                            </div>
                        </header>
                        <div class="post-content" inner_html=p.html.clone()></div>
                    </article>
                }.into_any(),
                None => view! { <p class="text-[var(--text-secondary)]">{move || tr(lang.get(), I18nKey::PostNotFound)}</p> }.into_any(),
            }}
        </div>
    }
}
