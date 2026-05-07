use crate::i18n::*;
use crate::stores::AppCtx;

use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    let AppCtx { app, .. } = expect_context();
    let lang = app.lang();

    view! {
        <section class="py-20 pb-16 overflow-hidden reveal">
            <div class="w-full max-w-[1120px] mx-auto px-6 max-sm:px-4 grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12 items-center">
                <div class="hero__content">
                    <span class="font-mono text-xs font-medium text-[var(--code-green)] block mb-3">{move || tr(lang.get(), I18nKey::HeroBlogLabel)}</span>
                    <h1 class="font-mono mb-4">
                        <span class="block text-3xl max-sm:text-2xl font-bold text-[var(--text-primary)]">{move || tr(lang.get(), I18nKey::HeroFnWriteIdeas)}</span>
                        <span class="block text-2xl max-sm:text-xl font-normal text-[var(--text-tertiary)] mt-1">
                            <span class="text-[var(--code-green)]">{move || tr(lang.get(), I18nKey::HeroArrowString)}</span>
                        </span>
                    </h1>
                    <p class="text-[var(--text-secondary)] text-lg max-w-[45ch] mb-8 leading-relaxed">{move || tr(lang.get(), I18nKey::HeroDescription)}</p>
                    <div class="flex gap-3 flex-wrap">
                        <a href="#posts-section"
                            class="
                                inline-flex items-center gap-2
                                font-mono text-sm font-medium
                                px-5 py-[10px] rounded-md
                                border border-transparent
                                cursor-pointer transition-all no-underline
                                leading-[1.4]
                                bg-[var(--brand-500)] text-white
                                hover:bg-[var(--brand-600)] hover:text-white
                                hover:-translate-y-[1px] active:translate-y-0
                            ">{move || tr(lang.get(), I18nKey::HeroPostsLatest)}</a>
                        <a href="/about"
                            class="
                                inline-flex items-center gap-2
                                font-mono text-sm font-medium
                                px-5 py-[10px] rounded-md
                                border border-transparent
                                cursor-pointer transition-all no-underline
                                leading-[1.4]
                                bg-transparent text-[var(--text-primary)]
                                hover:bg-[var(--brand-500)]/10 hover:text-[var(--brand-500)]
                            ">{move || tr(lang.get(), I18nKey::HeroAbout)}</a>
                    </div>
                </div>
                <div class="flex items-center justify-center">
                    <div class="w-full max-w-[420px] bg-[#0d1117] rounded-lg shadow-lg overflow-hidden border border-[#30363d] font-mono text-sm">
                        <div class="flex items-center gap-[6px] px-[14px] py-[10px] bg-[#161b22] border-b border-[#30363d]">
                            <span class="w-[10px] h-[10px] rounded-full bg-[#ff6b6b]"></span>
                            <span class="w-[10px] h-[10px] rounded-full bg-[#ffd93d]"></span>
                            <span class="w-[10px] h-[10px] rounded-full bg-[#6bcb77]"></span>
                            <span class="ml-auto text-xs text-[#8b949e] font-mono">"lektor.rs - vim"</span>
                        </div>
                        <div class="px-[18px] py-4 leading-[1.7]">
                            <span class="text-[var(--code-green)]">"$"</span>
                            <span class="text-[#e6edf3] ml-2">"cargo new blog"</span>
                            <div class="text-[#8b949e] my-1 mb-2 ml-[18px]">"Created binary (application) `blog` package"</div>
                            <span class="text-[var(--code-green)]">"$"</span>
                            <span class="text-[#e6edf3] ml-2 animate-pulse">"_"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
