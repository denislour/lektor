use crate::i18n::*;
use crate::stores::AppCtx;

use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    let AppCtx { app, .. } = expect_context();
    let lang = app.lang();

    view! {
        <div class="py-12 pb-16">
            <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4">
                <article class="max-w-[640px] mx-auto">
                    <h1 class="font-mono text-2xl font-semibold mb-8">{move || tr(lang.get(), I18nKey::NavAbout)}</h1>
                    <div class="font-body text-lg leading-[1.7] text-[var(--text-secondary)]">
                        <p class="mb-[1.25em]">{move || tr(lang.get(), I18nKey::AboutPara1)}</p>
                        <p class="mb-[1.25em]">{move || tr(lang.get(), I18nKey::AboutPara2)}</p>
                        <ul class="mb-[1.25em] ps-[1.5em] list-disc">
                            <li class="mb-[0.5em]">{move || tr(lang.get(), I18nKey::AboutItem1)}</li>
                            <li class="mb-[0.5em]">{move || tr(lang.get(), I18nKey::AboutItem2)}</li>
                            <li class="mb-[0.5em]">{move || tr(lang.get(), I18nKey::AboutItem3)}</li>
                            <li class="mb-[0.5em]">{move || tr(lang.get(), I18nKey::AboutItem4)}</li>
                            <li class="mb-[0.5em]">{move || tr(lang.get(), I18nKey::AboutItem5)}</li>
                        </ul>
                        <blockquote class="border-s-3 border-[var(--brand-500)] ps-4 my-[1.5em] italic">{move || tr(lang.get(), I18nKey::AboutBlockquote)}</blockquote>
                        <p class="mb-[1.25em]">{move || tr(lang.get(), I18nKey::AboutPara3)}</p>
                        <p class="mt-8 font-mono text-sm text-[var(--text-tertiary)]">{move || tr(lang.get(), I18nKey::AboutSignoff)}</p>
                    </div>
                </article>
            </div>
        </div>
    }
}
