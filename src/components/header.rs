use crate::i18n::*;
use crate::stores::AppCtx;
use leptos::prelude::*;

#[component]
pub fn Header(#[prop(optional)] show_search: bool) -> impl IntoView {
    let AppCtx { app, search, posts } = use_context().expect("AppCtx not provided");
    let lang = app.lang();
    view! {
        <header
            class="
                sticky top-0 z-100
                bg-[var(--surface-primary)]
                border-b border-[var(--border-subtle)]
                backdrop-blur-(12px)
                [-webkit-backdrop-filter:blur(12px)]
            "
            role="banner">
            <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4 flex items-center justify-between h-[60px]">
                <a href="/"
                    on:click=move |_| { app.set_page(1); }
                    class="
                        flex items-center gap-1
                        font-mono text-base font-semibold
                        text-[var(--text-primary)]
                        no-underline
                    "
                    aria-label={move || tr(lang.get(), I18nKey::BackToHomepage)}>
                    <span class="text-[var(--text-tertiary)] font-normal">"["</span>
                    <span class="">
                        "print"<span class="text-[var(--brand-500)]">"(\"Lektor\")"</span>
                    </span>
                    <span class="text-[var(--text-tertiary)] font-normal">"]"</span>
                </a>

                <nav class="flex-1 flex justify-center" aria-label="Main navigation">
                    <button
                        class="flex md:hidden flex-col gap-[5px] p-1 bg-transparent border-none cursor-pointer"
                        aria-label=move || if app.menu_open().get() { tr(lang.get(), I18nKey::CloseMenu) } else { tr(lang.get(), I18nKey::OpenMenu) }
                        aria-expanded=move || app.menu_open().get().to_string()
                        on:click=move |_| { app.toggle_menu(); }
                    >
                        <span class="block w-5 h-[2px] bg-[var(--text-primary)] rounded-[1px] transition-all"></span>
                        <span class="block w-5 h-[2px] bg-[var(--text-primary)] rounded-[1px] transition-all"></span>
                    </button>
                    <ul class=move || {
                        let base = "flex-col md:flex-row md:flex md:items-center absolute md:static top-[60px] left-0 right-0 bg-[var(--surface-primary)] md:bg-transparent border-b md:border-b-0 border-[var(--border-subtle)] p-4 md:p-0 gap-1";
                        if app.menu_open().get() {
                            format!("{} flex", base)
                        } else {
                            format!("{} hidden md:flex", base)
                        }
                    } role="list">
                        <li><a href="/" on:click=move |_| { app.set_page(1); } class="font-mono text-xs font-medium text-[var(--text-secondary)] hover:text-[var(--brand-500)] px-3 py-2 rounded-sm transition-all lowercase">{move || tr(lang.get(), I18nKey::NavHome)}</a></li>
                        <li><a href="/#posts-section" on:click=move |_| { app.set_page(1); } class="font-mono text-xs font-medium text-[var(--text-secondary)] hover:text-[var(--brand-500)] px-3 py-2 rounded-sm transition-all lowercase">{move || tr(lang.get(), I18nKey::NavPosts)}</a></li>
                        <li><a href="/about" class="font-mono text-xs font-medium text-[var(--text-secondary)] hover:text-[var(--brand-500)] px-3 py-2 rounded-sm transition-all lowercase">{move || tr(lang.get(), I18nKey::NavAbout)}</a></li>
                    </ul>
                </nav>
                {show_search.then(|| view! {
                    <div
                        class="
                            flex items-center gap-1
                            px-2 py-[2px]
                            border border-transparent rounded-sm
                            ml-auto self-center
                            transition-colors
                            focus-within:border-[var(--brand-500)]
                            focus-within:bg-[var(--surface-secondary)]
                        ">
                        <span class="font-mono text-xs text-[var(--code-green)] font-medium shrink-0">{move || tr(lang.get(), I18nKey::NavSearchPrompt)}</span>
                        <input type="text"
                            class="
                                font-mono text-xs
                                text-[var(--text-primary)]
                                bg-transparent border-none outline-none
                                w-[100px] focus:w-[160px]
                                transition-all p-0 leading-[1.4]
                                placeholder:text-[var(--text-tertiary)]
                            "
                            autocomplete="off"
                            prop:value=move || search.query().get()
                            on:input=move |ev| {
                                let input = event_target::<web_sys::HtmlInputElement>(&ev);
                                search.set_query(&input.value());
                            }
                        />
                        <span class="font-mono text-xs text-[var(--text-tertiary)] shrink-0">{move || {
                            let q = search.query().get();
                            if q.trim().is_empty() { return String::new(); }
                            let cnt = posts.items().get().len();
                            format!("{} {}", cnt, tr(lang.get(), I18nKey::SearchResultsLabel))
                        }}</span>
                    </div>
                })}

                <button
                    class="
                        bg-transparent
                        border border-[var(--border-subtle)]
                        cursor-pointer px-[10px] py-[4px] rounded-md
                        flex items-center justify-center
                        font-mono text-xs
                        transition-all leading-[1.4]
                        hover:border-[var(--brand-500)]
                        mr-2
                    "
                    on:click=move |_| {
                    app.toggle_lang();
                }>
                    <span class="text-[var(--text-tertiary)]">"["<span class="text-[var(--code-green)] font-medium">{move || if lang.get() == Lang::En { "en" } else { "vi" }}</span>"]"</span>
                </button>

                <button
                    class="
                        bg-transparent
                        border border-[var(--border-subtle)]
                        cursor-pointer px-[10px] py-[4px] rounded-md
                        flex items-center justify-center
                        font-mono text-xs
                        transition-all leading-[1.4]
                        hover:border-[var(--brand-500)]
                    "
                    aria-label="Toggle dark/light mode"
                    on:click=move |_| {
                        app.toggle_theme();
                    }
                >
                    <span class="text-[var(--text-tertiary)]">
                        "["
                        <span class="text-[var(--brand-500)] font-medium">{move || app.theme().get()}</span>
                        "]"
                    </span>
                </button>
            </div>
        </header>
    }
}
