use crate::components::get_started::CLAUDE_MCP;
use leptos::prelude::*;

#[component]
pub(crate) fn Hero() -> AnyView {
    view! {
        <section class="max-w-5xl mx-auto px-6 pt-16 pb-20 text-center">
            <div class="inline-block bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default text-warmgrey text-xs px-3 py-1 rounded-full mb-6 tracking-widest">
                "jebby says do a dance moves"
            </div>
            <h1 class="text-5xl sm:text-6xl font-black leading-tight mb-5">
                "get it together." <br /> <span class="text-amber">"jebby said so."</span>
            </h1>
            <p class="text-warmgrey text-lg sm:text-xl max-w-xl mx-auto mb-4 leading-relaxed">
                "set goals, manage tasks, ask jebby what's next."
            </p>
            <p
                class="text-warmblack font-bold text-lg mb-10"
                style="text-decoration: underline; text-decoration-color: #d97706; text-decoration-thickness: 3px; text-underline-offset: 4px;"
            >
                "why? because jebby says so, and that's enough."
            </p>
            <div class="flex flex-col sm:flex-row gap-3 justify-center items-center mb-10">
                <a
                    href="https://accounts.jebbysays.dev/sign-up"
                    class="bg-amber text-white font-extrabold px-7 py-3 rounded-full text-base hover:bg-amber-dark transition-colors shadow-md"
                >
                    "get started free →"
                </a>
                <a
                    href="https://github.com/josiahparry/jebbysays"
                    class="text-warmgrey font-bold px-7 py-3 rounded-full border border-warmgrey-light dark:border-border-default text-base hover:border-warmgrey hover:text-warmblack dark:hover:text-cream transition-colors"
                >
                    "self-host it"
                </a>
            </div>

            <div class="max-w-lg mx-auto">
                <div class="bg-code-bg rounded-2xl overflow-hidden dark:shadow-amber-glow">
                    <div class="flex items-center justify-between px-5 py-3 border-b border-white/10">
                        <div class="flex items-center gap-2">
                            <span class="w-3 h-3 rounded-full bg-red-400"></span>
                            <span class="w-3 h-3 rounded-full bg-yellow-400"></span>
                            <span class="w-3 h-3 rounded-full bg-green-400"></span>
                        </div>
                        <button
                            class="text-warmgrey hover:text-cream transition-colors p-1 rounded"
                            title="copy"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <rect x="9" y="9" width="13" height="13" rx="2" />
                                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
                            </svg>
                        </button>
                    </div>
                    <div class="px-6 py-5">
                        <code
                            class="font-normal text-sm leading-loose whitespace-pre text-left block"
                            style="color: #f5e6d0;"
                        >
                            {CLAUDE_MCP}
                        </code>
                    </div>
                </div>
                <p class="text-warmgrey text-sm text-center mt-4">
                    "sign up and 💩 done! jebby will help you, i promise."
                </p>
            </div>
        </section>
    }.into_any()
}
