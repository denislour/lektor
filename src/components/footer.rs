use crate::i18n::*;
use crate::stores::AppCtx;

use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let AppCtx { app, .. } = use_context().expect("AppCtx not provided");
    let lang = app.lang();

    view! {
        <footer class="border-t border-[var(--border-subtle)] py-12 pb-6" role="contentinfo">
            <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4 border-t border-[var(--border-subtle)] pt-6">
                <p class="font-mono text-xs text-[var(--text-tertiary)] text-center">
                    "2026 "
                    <span class="text-[var(--code-green)]">">"</span>
                    " "
                    {move || tr(lang.get(), I18nKey::FooterText)}
                </p>
            </div>
        </footer>
    }
}
