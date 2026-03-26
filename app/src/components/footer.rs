use leptos::prelude::*;

#[component]
pub(crate) fn Footer() -> AnyView {
    view! {
        <footer class="max-w-5xl mx-auto px-6 pb-10 flex flex-col sm:flex-row items-center justify-between gap-4 text-warmgrey text-sm">
            <span class="font-black text-warmblack dark:text-cream text-lg">
                "jebby" <span class="text-amber">"says."</span>
            </span>
            <div class="flex gap-5">
                <a
                    href="https://accounts.jebbysays.dev/sign-in"
                    class="hover:text-warmblack dark:hover:text-cream transition-colors"
                >
                    "sign in"
                </a>
                <a
                    href="https://accounts.jebbysays.dev/sign-up"
                    class="hover:text-warmblack dark:hover:text-cream transition-colors"
                >
                    "sign up"
                </a>
                <a
                    href="https://github.com/josiahparry/jebbysays"
                    class="hover:text-warmblack dark:hover:text-cream transition-colors"
                >
                    "github"
                </a>
            </div>
            <span class="text-xs text-warmgrey-light">
                "made with 🧡 for jebby, juice & beanie"
            </span>
        </footer>
    }.into_any()
}
