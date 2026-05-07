use crate::i18n::*;

use crate::stores::AppCtx;
use leptos::prelude::*;

fn page_btn(num: usize, active: bool, set_page: impl Fn(usize) + Copy + 'static) -> AnyView {
    let base = "w-9 h-9 rounded-sm cursor-pointer font-inherit text-inherit transition-all";
    let cls = if active {
        format!(
            "{} bg-[var(--brand-500)] text-white border-[var(--brand-500)]",
            base
        )
    } else {
        format!(
            "{} bg-transparent border border-transparent text-[var(--text-secondary)] hover:border-[var(--border-primary)]",
            base
        )
    };
    view! {
        <button class=cls on:click=move |_| { set_page(num); }>
            {num}
        </button>
    }
    .into_any()
}

fn dots_view() -> AnyView {
    view! { <span class="text-[var(--text-tertiary)] text-sm tracking-[2px] px-1 select-none">".."</span> }.into_any()
}

#[component]
pub fn Pagination(
    page: impl Get<Value = usize> + Copy + Send + 'static,
    on_page_change: impl Fn(usize) + Copy + Send + 'static,
    total: impl Fn() -> usize + 'static + Copy + Send,
) -> impl IntoView {
    let AppCtx { app, .. } = use_context().expect("AppCtx not provided");
    let lang = app.lang();

    move || {
        let total = total();
        let cur = page.get();
        if total <= 1 {
            return ().into_any();
        }

        let set_pg = on_page_change;
        let delta: isize = 1;
        let left = cur as isize - delta;
        let right = cur as isize + delta + 1;

        let mut range: Vec<usize> = Vec::new();
        for i in 1..=total {
            if i == 1 || i == total || ((i as isize) >= left && (i as isize) < right) {
                range.push(i);
            }
        }

        let mut page_items: Vec<AnyView> = Vec::new();
        let mut last: Option<usize> = None;
        for &i in &range {
            if let Some(l) = last {
                if i - l == 2 {
                    page_items.push(page_btn(l + 1, false, set_pg));
                } else if i - l != 1 {
                    page_items.push(dots_view());
                }
            }
            page_items.push(page_btn(i, i == cur, set_pg));
            last = Some(i);
        }

        view! {
            <div class="flex items-center justify-center gap-2 mt-10 font-mono text-sm">
                <button
                    class="
                        bg-transparent
                        border border-[var(--border-primary)]
                        text-[var(--text-secondary)] px-4 py-2 rounded-sm
                        cursor-pointer font-inherit text-inherit
                        transition-all
                        hover:enabled:border-[var(--brand-500)]
                        hover:enabled:text-[var(--brand-500)]
                        disabled:opacity-30 disabled:cursor-not-allowed
                    "
                    disabled={cur <= 1}
                    on:click=move |_| { on_page_change((page.get() - 1).max(1)); }>
                    {move || tr(lang.get(), I18nKey::PaginationPrev)}
                </button>
                {page_items}
                <button
                    class="
                        bg-transparent
                        border border-[var(--border-primary)]
                        text-[var(--text-secondary)] px-4 py-2 rounded-sm
                        cursor-pointer font-inherit text-inherit
                        transition-all
                        hover:enabled:border-[var(--brand-500)]
                        hover:enabled:text-[var(--brand-500)]
                        disabled:opacity-30 disabled:cursor-not-allowed
                    "
                    disabled={cur >= total}
                    on:click=move |_| { on_page_change((page.get() + 1).min(total)); }>
                    {move || tr(lang.get(), I18nKey::PaginationNext)}
                </button>
            </div>
        }
        .into_any()
    }
}
