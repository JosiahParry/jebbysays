use leptos::prelude::*;

#[component]
pub(crate) fn HowItWorks() -> AnyView {
    view! {
        <section class="max-w-5xl mx-auto px-6 py-20">
            <h2 class="text-3xl font-black text-center mb-2">"how it works 🐾"</h2>
            <p class="text-warmgrey text-center mb-12">
                "jebby keeps it simple. you just have to show up."
            </p>
            <div class="grid sm:grid-cols-3 gap-8">
                <div class="bg-warmgrey-pale rounded-2xl p-7 flex flex-col gap-3">
                    <div class="text-3xl">"🎯"</div>
                    <h3 class="font-black text-lg">"tell jebby your goals"</h3>
                    <p class="text-warmgrey text-sm leading-relaxed">
                        "write your goals in plain language. give them a title, as much context as you want, and a priority from 1 to 5. jebby will remember every word."
                    </p>
                </div>
                <div class="bg-amber/10 border border-amber/20 rounded-2xl p-7 flex flex-col gap-3">
                    <div class="text-3xl">"✅"</div>
                    <h3 class="font-black text-lg">"break it into tasks"</h3>
                    <p class="text-warmgrey text-sm leading-relaxed">
                        "attach tasks to your goals. set deadlines, priorities, tags, and context. jebby doesn't let things fall through the cracks\u{2014}she's built different."
                    </p>
                </div>
                <div class="bg-warmgrey-pale rounded-2xl p-7 flex flex-col gap-3">
                    <div class="text-3xl">"💬"</div>
                    <h3 class="font-black text-lg">"do what jebby says"</h3>
                    <p class="text-warmgrey text-sm leading-relaxed">
                        "ask for your daily briefing. run a weekly retro. ask \"what should I focus on?\" when jebby tells you to do a dance move, you do the dance moves."
                    </p>
                </div>
            </div>
        </section>
    }.into_any()
}
