use crate::components::navbar::{get_user_id, Navbar};
use icondata as id;
use leptos::prelude::*;
use leptos_icons::Icon;

#[cfg(feature = "ssr")]
use {
    jebbysays_core::auth::web::SESSION_KEY_USER_ID, jebbysays_core::portfolio::Portfolio,
    jebbysays_core::McpState, std::sync::Arc, tower_sessions::Session,
};

#[server]
pub async fn get_objectives() -> Result<Vec<jebbysays_core::types::Objective>, ServerFnError> {
    let session = leptos_axum::extract::<Session>().await?;
    let user_id: String = session
        .get(SESSION_KEY_USER_ID)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("not logged in"))?;

    let mcp = expect_context::<Arc<McpState>>();
    Portfolio::with_user(mcp.db.clone(), user_id)
        .list_objectives()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_incomplete_tasks() -> Result<Vec<jebbysays_core::types::Task>, ServerFnError> {
    let session = leptos_axum::extract::<Session>().await?;
    let user_id: String = session
        .get(SESSION_KEY_USER_ID)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("not logged in"))?;

    let mcp = expect_context::<Arc<McpState>>();
    Portfolio::with_user(mcp.db.clone(), user_id)
        .list_incomplete_tasks()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

fn priority_badge(p: i64) -> &'static str {
    match p {
        5 => "bg-red-100 text-red-700",
        4 => "bg-orange-100 text-orange-700",
        3 => "bg-amber-100 text-amber-700",
        _ => "bg-warmgrey-pale text-warmgrey",
    }
}

fn priority_label(p: i64) -> &'static str {
    match p {
        5 => "p5",
        4 => "p4",
        3 => "p3",
        2 => "p2",
        _ => "p1",
    }
}

#[component]
fn NotLoggedIn() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-[60vh] text-center px-6">
            <div class="text-6xl mb-6">"💅"</div>
            <h2 class="text-3xl font-black text-warmblack mb-3">
                "jebby says: " <span class="text-amber">"sign in first!"</span>
            </h2>
            <p class="text-warmgrey text-lg max-w-sm mb-2">
                "you can't see your stuff if jebby doesn't know who you are."
            </p>
            <p class="text-warmgrey text-sm max-w-sm mb-8">
                "jebby has standards. they're low, but they exist."
            </p>
            <a
                href="/auth/login"
                rel="external"
                class="bg-amber text-white font-extrabold px-7 py-3 rounded-full text-base hover:bg-amber-dark transition-colors shadow-md"
            >
                "let's go →"
            </a>
        </div>
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let user = Resource::new(|| (), |_| get_user_id());
    let objectives = Resource::new(|| (), |_| get_objectives());
    let tasks = Resource::new(|| (), |_| get_incomplete_tasks());

    view! {
        <div class="min-h-screen bg-cream">
            <Navbar />
            <Suspense>
                {move || match user.get() {
                    Some(Ok(None)) | Some(Err(_)) => view! { <NotLoggedIn /> }.into_any(),
                    _ => {
                        view! {
                            <div class="max-w-5xl mx-auto px-6 py-10">
                                <div class="mb-8">
                                    <h1 class="text-3xl font-black text-warmblack">
                                        "what's the move?"
                                    </h1>
                                    <p class="text-warmgrey mt-1">
                                        "jebby's got your back. here's where things stand."
                                    </p>
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    // Objectives column
                                    <div>
                                        <div class="flex items-center gap-2 mb-4">
                                            <span class="text-amber">
                                                <Icon icon=id::LuTarget width="18" height="18" />
                                            </span>
                                            <h2 class="font-black text-lg text-warmblack">
                                                "objectives"
                                            </h2>
                                            <Suspense>
                                                {move || {
                                                    objectives
                                                        .get()
                                                        .and_then(|r: Result<Vec<_>, _>| r.ok())
                                                        .map(|o| {
                                                            view! {
                                                                <span class="ml-auto text-xs text-warmgrey bg-warmgrey-pale px-2 py-0.5 rounded-full">
                                                                    {o.len()} " total"
                                                                </span>
                                                            }
                                                        })
                                                }}
                                            </Suspense>
                                        </div>
                                        <Suspense fallback=|| {
                                            view! {
                                                <div class="text-warmgrey text-sm">"loading..."</div>
                                            }
                                        }>
                                            {move || match objectives.get() {
                                                None => view! { <div /> }.into_any(),
                                                Some(Err(e)) => {
                                                    view! {
                                                        <div class="text-red-500 text-sm">{e.to_string()}</div>
                                                    }
                                                        .into_any()
                                                }
                                                Some(Ok(objs)) if objs.is_empty() => {
                                                    view! {
                                                        <div class="bg-warmgrey-pale rounded-2xl p-6 text-center text-warmgrey text-sm">
                                                            <span class="flex justify-center mb-2 text-amber">
                                                                <Icon icon=id::LuSparkles width="24" height="24" />
                                                            </span>
                                                            "no objectives yet. ask jebby to help you set some up!"
                                                        </div>
                                                    }
                                                        .into_any()
                                                }
                                                Some(Ok(objs)) => {
                                                    view! {
                                                        <div class="flex flex-col gap-3">
                                                            {objs
                                                                .into_iter()
                                                                .map(|obj| {
                                                                    view! {
                                                                        <div class="bg-white rounded-2xl p-4 shadow-sm border border-warmgrey-light hover:border-amber transition-colors">
                                                                            <div class="flex items-start justify-between gap-2">
                                                                                <p class="font-bold text-warmblack text-sm leading-snug">
                                                                                    {obj.title}
                                                                                </p>
                                                                                <span class=format!(
                                                                                    "shrink-0 text-xs font-bold px-2 py-0.5 rounded-full {}",
                                                                                    priority_badge(obj.priority),
                                                                                )>{priority_label(obj.priority)}</span>
                                                                            </div>
                                                                            {obj
                                                                                .context
                                                                                .map(|c| {
                                                                                    view! {
                                                                                        <p class="text-warmgrey text-xs mt-1.5 leading-relaxed line-clamp-2">
                                                                                            {c}
                                                                                        </p>
                                                                                    }
                                                                                })}
                                                                        </div>
                                                                    }
                                                                })
                                                                .collect_view()}
                                                        </div>
                                                    }
                                                        .into_any()
                                                }
                                            }}
                                        </Suspense>
                                    </div>

                                    // Tasks column
                                    <div>
                                        <div class="flex items-center gap-2 mb-4">
                                            <span class="text-amber">
                                                <Icon icon=id::LuListTodo width="18" height="18" />
                                            </span>
                                            <h2 class="font-black text-lg text-warmblack">
                                                "open tasks"
                                            </h2>
                                            <Suspense>
                                                {move || {
                                                    tasks
                                                        .get()
                                                        .and_then(|r: Result<Vec<_>, _>| r.ok())
                                                        .map(|t| {
                                                            view! {
                                                                <span class="ml-auto text-xs text-warmgrey bg-warmgrey-pale px-2 py-0.5 rounded-full">
                                                                    {t.len()} " open"
                                                                </span>
                                                            }
                                                        })
                                                }}
                                            </Suspense>
                                        </div>
                                        <Suspense fallback=|| {
                                            view! {
                                                <div class="text-warmgrey text-sm">"loading..."</div>
                                            }
                                        }>
                                            {move || match tasks.get() {
                                                None => view! { <div /> }.into_any(),
                                                Some(Err(e)) => {
                                                    view! {
                                                        <div class="text-red-500 text-sm">{e.to_string()}</div>
                                                    }
                                                        .into_any()
                                                }
                                                Some(Ok(ts)) if ts.is_empty() => {
                                                    view! {
                                                        <div class="bg-warmgrey-pale rounded-2xl p-6 text-center text-warmgrey text-sm">
                                                            <span class="flex justify-center mb-2 text-amber">
                                                                <Icon icon=id::LuPartyPopper width="24" height="24" />
                                                            </span>
                                                            "you're all caught up! jebby is proud."
                                                        </div>
                                                    }
                                                        .into_any()
                                                }
                                                Some(Ok(ts)) => {
                                                    view! {
                                                        <div class="flex flex-col gap-3">
                                                            {ts
                                                                .into_iter()
                                                                .map(|task| {
                                                                    view! {
                                                                        <div class="bg-white rounded-2xl p-4 shadow-sm border border-warmgrey-light hover:border-amber transition-colors">
                                                                            <div class="flex items-start justify-between gap-2">
                                                                                <p class="font-bold text-warmblack text-sm leading-snug">
                                                                                    {task.title}
                                                                                </p>
                                                                                <span class=format!(
                                                                                    "shrink-0 text-xs font-bold px-2 py-0.5 rounded-full {}",
                                                                                    priority_badge(task.priority),
                                                                                )>{priority_label(task.priority)}</span>
                                                                            </div>
                                                                            {task
                                                                                .deadline
                                                                                .map(|d: jiff::Timestamp| {
                                                                                    view! {
                                                                                        <div class="flex items-center gap-1 mt-1.5 text-xs text-warmgrey">
                                                                                            <Icon icon=id::LuCalendarClock width="12" height="12" />
                                                                                            {d.strftime("%b %d").to_string()}
                                                                                        </div>
                                                                                    }
                                                                                })}
                                                                            {task
                                                                                .tags
                                                                                .filter(|t: &Vec<String>| !t.is_empty())
                                                                                .map(|tags: Vec<String>| {
                                                                                    view! {
                                                                                        <div class="flex gap-1 flex-wrap mt-2">
                                                                                            {tags
                                                                                                .into_iter()
                                                                                                .map(|tag| {
                                                                                                    view! {
                                                                                                        <span class="text-xs bg-warmgrey-pale text-warmgrey px-2 py-0.5 rounded-full">
                                                                                                            {tag}
                                                                                                        </span>
                                                                                                    }
                                                                                                })
                                                                                                .collect_view()}
                                                                                        </div>
                                                                                    }
                                                                                })}
                                                                        </div>
                                                                    }
                                                                })
                                                                .collect_view()}
                                                        </div>
                                                    }
                                                        .into_any()
                                                }
                                            }}
                                        </Suspense>
                                    </div>
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
