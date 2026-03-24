use leptos::prelude::*;

#[component]
pub(crate) fn WhoIsJebby() -> AnyView {
    view! {
        <section class="max-w-3xl mx-auto px-6 py-20 text-center">
            <h2 class="text-3xl font-black mb-6">"so...who's jebby?"</h2>
            <p class="text-warmgrey text-lg leading-relaxed mb-4">
                "jebby is my wife's nickname. we both have crippling ADHD. but she has this superpower: she is "
                <span class="text-warmblack font-extrabold">
                    "unbelievably organized, detail-oriented,"
                </span> " and when she sets her mind to something?"
            </p>
            <p class="text-warmgrey text-lg leading-relaxed mb-4">
                "so when jebby says \"do a dance move\"\u{2014}you do a dance move. you will be better for it."
            </p>
            <p class="text-warmgrey text-lg leading-relaxed">
                "we also have two cats. " <span class="text-warmblack font-bold">"Juice"</span>
                " is a tuxedo cat with a red collar\u{2014}the angel on her shoulder. good boy. very official. "
                <span class="text-warmblack font-bold">"Beanie"</span>
                " (government name: Onion) is a tortie with one paw the color of peanut butter\u{2014}the devil on her shoulder. chaos agent. they both supervised the building of this app and have strong opinions."
            </p>
            <div class="mt-8 text-4xl">"🐱🐾🐱"</div>
        </section>
    }.into_any()
}
