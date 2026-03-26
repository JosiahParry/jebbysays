use leptos::prelude::*;

#[component]
pub(crate) fn Divider() -> AnyView {
    view! {
        <div class="max-w-5xl mx-auto px-6">
            <div class="border-t border-warmgrey-light dark:border-border-default"></div>
        </div>
    }
    .into_any()
}
