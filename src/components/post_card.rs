use crate::data::{Post, format_date};
use crate::i18n::*;
use crate::stores::AppCtx;

use crate::utils::{save_scroll_pos, tag_to_lang};
use leptos::prelude::*;

#[component]
pub fn PostCard(post: Post) -> impl IntoView {
    let AppCtx { app, .. } = expect_context();
    let lang = app.lang();
    let href = format!("/post?id={}", post.id);
    let prog_lang = post
        .tags
        .first()
        .map(|t| tag_to_lang(t))
        .unwrap_or("plaintext")
        .to_string();

    view! {
        <article
            class="
                bg-[var(--surface-card)] rounded-lg overflow-hidden shadow-sm
                hover:shadow-lg hover:-translate-y-[3px] transition-all
                flex flex-col
                border border-[var(--border-subtle)] hover:border-[var(--border-primary)]
            ">
            <div
                class="
                    relative overflow-hidden bg-[#0d1117]
                    flex items-center justify-center
                    h-[260px] shrink-0
                ">
                <div class="w-full p-3">
                    <pre
                        style="
                            mask-image:linear-gradient(to bottom,black 60%,transparent 100%);
                            -webkit-mask-image:linear-gradient(to bottom,black 60%,transparent 100%)
                        "
                        class="
                            text-[#e6edf3] font-mono text-[0.85rem]
                            leading-[1.4] overflow-hidden
                            whitespace-pre max-h-[200px]
                        "><code class={format!("language-{}", prog_lang)}>{post.thumbnail_code.clone()}</code></pre>
                </div>
                <div class="absolute bottom-2 right-2 font-mono text-[0.625rem] text-[var(--code-green)] bg-[var(--code-green)]/15 px-[6px] py-[1px] rounded-[3px]">{prog_lang.clone()}</div>
            </div>
            <div class="p-6 flex flex-col gap-3 flex-1">
                <div class="font-mono text-xs text-[var(--text-tertiary)] flex items-center gap-2 flex-wrap">
                    <time datetime={post.date.clone()}>{format_date(&post.date)}</time>
                    <span>" · " {post.read_time} " "{move || tr(lang.get(), I18nKey::PostCardMinRead)}</span>
                </div>
                <h3 class="font-mono text-base font-semibold leading-tight">
                    <a href=href.clone() class="text-[var(--text-primary)] hover:text-[var(--brand-500)]" on:click=|_| save_scroll_pos()>{post.title.clone()}</a>
                </h3>
                <p class="text-[var(--text-secondary)] text-sm flex-1 leading-normal">{post.excerpt.clone()}</p>
                <div class="flex items-center justify-between mt-auto pt-3">
                    <div class="flex flex-wrap gap-2">
                        {post.tags.iter().map(|t| view! { <span class="inline-block text-[0.6875rem] font-medium text-[var(--brand-500)] bg-[var(--brand-50)] dark:bg-[var(--brand-500)]/15 px-2 py-[2px] rounded font-mono lowercase">{t.clone()}</span> }).collect::<Vec<_>>()}
                    </div>
                    <a href=href.clone()
                        class="
                            font-mono text-xs font-medium
                            text-[var(--brand-500)]
                            whitespace-nowrap transition-all
                            inline-flex items-center gap-1 hover:gap-2
                        "
                        on:click=|_| save_scroll_pos()>{move || tr(lang.get(), I18nKey::PostCardRead)}</a>
                </div>
            </div>
        </article>
    }
}
