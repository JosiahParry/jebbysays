use leptos::prelude::*;

#[component]
pub(crate) fn WhyJebby() -> AnyView {
    view! {
        <section class="max-w-5xl mx-auto px-6 py-20">
            <h2 class="text-3xl font-black text-center mb-2">"why jebby says?"</h2>
            <p class="text-warmgrey text-center mb-10">"maybe you'll recognize yourself here."</p>
            <ul class="max-w-lg mx-auto flex flex-col gap-5">
                <li class="flex gap-4 items-start">
                    <span class="text-xl mt-0.5">"🐾"</span>
                    <span class="text-lg leading-relaxed">
                        "do you, like me, have crippling ADHD?"
                    </span>
                </li>
                <li class="flex gap-4 items-start">
                    <span class="text-xl mt-0.5">"🐾"</span>
                    <span class="text-lg leading-relaxed">
                        "do you need an accountability buddy who won't let you off the hook?"
                    </span>
                </li>
                <li class="flex gap-4 items-start">
                    <span class="text-xl mt-0.5">"🐾"</span>
                    <span class="text-lg leading-relaxed">
                        "do you struggle to keep track of your own todo list?"
                    </span>
                </li>
            </ul>
        </section>
    }.into_any()
}
