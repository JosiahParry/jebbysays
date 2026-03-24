use icondata as id;
use leptos::prelude::*;
use leptos_icons::Icon;

#[server]
pub async fn get_user_id() -> Result<Option<String>, ServerFnError> {
    use tower_sessions::Session;
    let session = leptos_axum::extract::<Session>().await?;
    let user_id = session
        .get::<String>("user_id")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(user_id)
}

#[component]
pub(crate) fn Navbar() -> AnyView {
    let user = Resource::new(|| (), |_| get_user_id());

    view! {
        <nav class="sticky top-0 z-50 bg-cream/95 backdrop-blur border-b border-warmgrey-light">
            <div class="flex items-center justify-between px-6 py-3 max-w-5xl mx-auto">
                <a
                    href="/"
                    class="text-xl font-black tracking-tight text-warmblack flex items-center gap-1"
                >
                    "jebby"
                    <span class="text-amber">"says."</span>
                </a>
                <div class="flex gap-2 items-center">
                    <Suspense>
                        {move || match user.get() {
                            Some(Ok(Some(_))) => {
                                view! {
                                    <a
                                        href="/dashboard"
                                        class="flex items-center gap-1.5 text-sm text-warmgrey hover:text-warmblack transition-colors px-3 py-1.5 rounded-lg hover:bg-warmgrey-pale"
                                    >
                                        <Icon icon=id::LuLayoutDashboard width="15" height="15" />
                                        "dashboard"
                                    </a>
                                    <a
                                        href="/auth/logout"
                                        rel="external"
                                        class="flex items-center gap-1.5 text-sm text-warmgrey hover:text-warmblack transition-colors px-3 py-1.5 rounded-lg hover:bg-warmgrey-pale"
                                    >
                                        <Icon icon=id::LuLogOut width="15" height="15" />
                                        "sign out"
                                    </a>
                                }
                                    .into_any()
                            }
                            _ => {
                                view! {
                                    <a
                                        href="/auth/login"
                                        rel="external"
                                        class="text-sm text-warmgrey hover:text-warmblack transition-colors px-3 py-1.5 rounded-lg hover:bg-warmgrey-pale"
                                    >
                                        "sign in"
                                    </a>
                                    <a
                                        href="/auth/login"
                                        rel="external"
                                        class="text-sm font-extrabold bg-amber text-white px-4 py-2 rounded-full hover:bg-amber-dark transition-colors shadow-sm"
                                    >
                                        "get started"
                                    </a>
                                }
                                    .into_any()
                            }
                        }}
                    </Suspense>
                </div>
            </div>
        </nav>
    }.into_any()
}
