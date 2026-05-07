use crate::data::format_date;
use crate::i18n::*;
use crate::stores::AppCtx;

use crate::utils::{save_scroll_pos, tag_to_lang};
use leptos::prelude::*;

#[component]
pub fn FeaturedPost() -> impl IntoView {
    let AppCtx { app, posts, .. } = use_context().expect("AppCtx not provided");
    let lang = app.lang();

    move || {
        let post = posts.latest();

        let Some(post) = post else {
            return ().into_any();
        };

        let prog_lang = post
            .tags
            .first()
            .map(|t| tag_to_lang(t))
            .unwrap_or("plaintext");

        view! {
            <section class="pb-16 reveal" id="featured-section">
                <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4">
                    <div class="grid grid-cols-1 md:grid-cols-[1.1fr_1fr] bg-[var(--surface-card)] rounded-xl overflow-hidden shadow-md hover:shadow-lg transition-shadow">
                        <div class="relative min-h-[340px] overflow-hidden bg-[#0d1117] flex items-center justify-center">
                            <div class="w-full p-5 relative">
                                <pre><code class={format!("language-{}", prog_lang)}>{post.thumbnail_code.as_str()}</code></pre>
                                <div
                                    class="
                                        absolute top-3 right-3
                                        font-mono text-[0.6875rem]
                                        text-[var(--code-green)]
                                        bg-[var(--code-green)]/15
                                        px-2 py-[2px] rounded
                                    ">{prog_lang}</div>
                            </div>
                        </div>
                        <div class="p-10 max-md:p-6">
                            <div class="font-mono text-xs text-[var(--text-tertiary)] mb-3 flex items-center gap-2 flex-wrap">
                                <span
                                    class="
                                        inline-block text-[0.6875rem] font-medium
                                        px-2 py-[2px] rounded font-mono lowercase
                                        bg-[var(--code-green)] text-white
                                        dark:bg-[var(--code-green)]/20 dark:text-[var(--code-green)]
                                    ">{move || tr(lang.get(), I18nKey::FeaturedBadge)}</span>
                                <time datetime={post.date.as_str()}>{format_date(&post.date)}</time>
                                <span>" · " {post.read_time} " "{move || tr(lang.get(), I18nKey::MinRead)}</span>
                            </div>
                            <h2 class="font-mono text-xl font-semibold leading-tight mb-3">
                                <a href={format!("/post?id={}", post.id)} class="text-[var(--text-primary)] hover:text-[var(--brand-500)]"
                                    on:click=|_| save_scroll_pos()>{post.title.as_str()}</a>
                            </h2>
                            <p class="text-[var(--text-secondary)] mb-4">{post.excerpt.as_str()}</p>
                            <div class="flex gap-2 mb-4">
                                {post.tags.iter().map(|t| view! { <span class="inline-block text-[0.6875rem] font-medium text-[var(--brand-500)] bg-[var(--brand-50)] dark:bg-[var(--brand-500)]/15 px-2 py-[2px] rounded font-mono lowercase">{t.as_str()}</span> }).collect::<Vec<_>>()}
                            </div>
                            <a href={format!("/post?id={}", post.id)}
                                class="
                                    font-mono text-sm font-medium
                                    text-[var(--brand-500)]
                                    inline-flex items-center gap-1 hover:gap-2
                                "
                                on:click=|_| save_scroll_pos()>{move || tr(lang.get(), I18nKey::ReadMoreArrow)}</a>
                        </div>
                    </div>
                </div>
            </section>
        }.into_any()
    }
}
