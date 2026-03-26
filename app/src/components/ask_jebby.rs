use leptos::prelude::*;

#[component]
pub(crate) fn AskJebby() -> AnyView {
    view! {
        <section class="max-w-5xl mx-auto px-6 py-20">
            <h2 class="text-3xl font-black text-center mb-2">"ask jebby anything"</h2>
            <p class="text-warmgrey text-center mb-12">
                "she will not let you spiral. trust the process."
            </p>
            <div class="grid sm:grid-cols-2 gap-5">
                <div class="flex gap-4 items-start p-5 rounded-2xl hover:bg-warmgrey-pale dark:hover:bg-surface-subtle transition-colors">
                    <span class="text-2xl mt-0.5">"☀️"</span>
                    <div>
                        <h4 class="font-extrabold mb-1">"daily briefing"</h4>
                        <p class="text-warmgrey text-sm leading-relaxed">
                            "Start your morning knowing exactly what matters today. Jebby has already sorted it out while you were still in bed."
                        </p>
                    </div>
                </div>
                <div class="flex gap-4 items-start p-5 rounded-2xl hover:bg-warmgrey-pale dark:hover:bg-surface-subtle transition-colors">
                    <span class="text-2xl mt-0.5">"🔭"</span>
                    <div>
                        <h4 class="font-extrabold mb-1">"week ahead"</h4>
                        <p class="text-warmgrey text-sm leading-relaxed">
                            "Sunday scaries? Jebby's got you. See what's coming, spot the conflicts, and start the week like someone who has their life together."
                        </p>
                    </div>
                </div>
                <div class="flex gap-4 items-start p-5 rounded-2xl hover:bg-warmgrey-pale dark:hover:bg-surface-subtle transition-colors">
                    <span class="text-2xl mt-0.5">"🪞"</span>
                    <div>
                        <h4 class="font-extrabold mb-1">"weekly retro"</h4>
                        <p class="text-warmgrey text-sm leading-relaxed">
                            "Look back. Celebrate what got done. Understand what didn't. Jebby will be honest with you—lovingly, but honest."
                        </p>
                    </div>
                </div>
                <div class="flex gap-4 items-start p-5 rounded-2xl hover:bg-warmgrey-pale dark:hover:bg-surface-subtle transition-colors">
                    <span class="text-2xl mt-0.5">"🧠"</span>
                    <div>
                        <h4 class="font-extrabold mb-1">"what should I do right now?"</h4>
                        <p class="text-warmgrey text-sm leading-relaxed">
                            "Just ask. Jebby knows your goals, your deadlines, and your priorities. She'll tell you. You'll do it."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }.into_any()
}
