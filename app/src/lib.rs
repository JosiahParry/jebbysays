use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub mod components;
use components::dashboard::DashboardPage;
use components::{
    ask_jebby::AskJebby, divider::Divider, footer::Footer, get_started::GetStarted, hero::Hero,
    how_it_works::HowItWorks, navbar::Navbar, who_is_jebby::WhoIsJebby, why_jebby::WhyJebby,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
                <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> AnyView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jebbysays.css" />
        <Title text="jebbysays — do a dance moves" />
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage />
                <Route path=StaticSegment("dashboard") view=DashboardPage />
            </Routes>
        </Router>
    }
    .into_any()
}

#[component]
fn HomePage() -> AnyView {
    view! {
        <div class="bg-cream text-warmblack">
            <Navbar />
            <Hero />
            <Divider />
            <WhyJebby />
            <Divider />
            <AskJebby />
            <Divider />
            <GetStarted />
            <Divider />
            <HowItWorks />
            <Divider />
            <WhoIsJebby />
            <Divider />
            <Footer />
        </div>
    }
    .into_any()
}
