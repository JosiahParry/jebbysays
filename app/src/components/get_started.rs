use leptos::prelude::*;

pub(crate) const CLAUDE_MCP: &str = r#"claude mcp add jebbysays \
    --scope user \
    --transport http \
    https://jebbysays.dev/mcp"#;

#[component]
pub(crate) fn GetStarted() -> AnyView {
    view! {
        <section class="max-w-5xl mx-auto px-6 py-20">
            <h2 class="text-3xl font-black text-center mb-2">"get started"</h2>
            <p class="text-warmgrey text-center mb-12">"three steps, then jebby takes over."</p>

            <ol class="max-w-xl mx-auto flex flex-col gap-6 mb-14">
                <li class="flex gap-5 items-start">
                    <span class="bg-amber text-white font-black text-sm w-8 h-8 rounded-full flex items-center justify-center shrink-0 mt-0.5">
                        "1"
                    </span>
                    <div class="flex-1">
                        <p class="font-extrabold mb-1">"sign up at jebbysays.dev"</p>
                        <p class="text-warmgrey text-sm leading-relaxed">
                            "create your free account at "
                            <a
                                href="https://accounts.jebbysays.dev/sign-up"
                                class="text-amber hover:text-amber-dark transition-colors font-bold"
                            >
                                "jebbysays.dev"
                            </a> "."
                        </p>
                    </div>
                </li>
                <li class="flex gap-5 items-start">
                    <span class="bg-amber text-white font-black text-sm w-8 h-8 rounded-full flex items-center justify-center shrink-0 mt-0.5">
                        "2"
                    </span>
                    <div class="flex-1">
                        <p class="font-extrabold mb-1">"connect via Claude Code"</p>
                        <p class="text-warmgrey text-sm leading-relaxed mb-3">
                            "run this once in your terminal:"
                        </p>
                        <div class="bg-code-bg rounded-xl overflow-hidden dark:shadow-amber-glow">
                            <div class="flex items-center justify-between px-4 py-2.5 border-b border-white/10">
                                <div class="flex items-center gap-1.5">
                                    <span class="w-3 h-3 rounded-full bg-red-400"></span>
                                    <span class="w-3 h-3 rounded-full bg-yellow-400"></span>
                                    <span class="w-3 h-3 rounded-full bg-green-400"></span>
                                </div>
                            </div>
                            <div class="px-5 py-4">
                                <code
                                    class="font-normal text-sm leading-loose whitespace-pre text-left block"
                                    style="color: #f5e6d0;"
                                >
                                    {CLAUDE_MCP}
                                </code>
                            </div>
                        </div>
                    </div>
                </li>
                <li class="flex gap-5 items-start">
                    <span class="bg-amber text-white font-black text-sm w-8 h-8 rounded-full flex items-center justify-center shrink-0 mt-0.5">
                        "3"
                    </span>
                    <div class="flex-1">
                        <p class="font-extrabold mb-1">"authenticate"</p>
                        <p class="text-warmgrey text-sm leading-relaxed">
                            "open Claude Code with "
                            <code class="bg-warmgrey-pale dark:bg-surface-subtle px-1.5 py-0.5 rounded text-xs font-bold">
                                "claude"
                            </code> ", " "type "
                            <code class="bg-warmgrey-pale dark:bg-surface-subtle px-1.5 py-0.5 rounded text-xs font-bold">
                                "/mcp"
                            </code> ", "
                            "find jebby says, and hit authenticate. that's it. jebby's ready."
                        </p>
                    </div>
                </li>
            </ol>

            <h3 class="text-xl font-black text-center mb-2">"now just talk to jebby"</h3>
            <p class="text-warmgrey text-center text-sm mb-12">
                "copy any of these straight into Claude Code to get going."
            </p>

            <div class="max-w-3xl mx-auto flex flex-col gap-12">

                <div class="grid sm:grid-cols-2 gap-6 items-center">
                    <div>
                        <p class="text-xs font-extrabold uppercase tracking-widest text-amber mb-4">
                            "🎯 set up your goals"
                        </p>
                        <div class="flex flex-col gap-3">
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "I'd like jebby's help getting organized! can you help me create some objectives? for each one, ask me for a title, any additional context that would help my LLM understand the goal, and a priority between 1 and 5."
                                </p>
                            </div>
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "what are my current objectives? show me how they're prioritized and give me a quick summary of each one."
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="mt-8 sm:mt-0">
                        <img
                            src="/imgs/jebby-prompt1.png"
                            alt="setting up objectives with jebby in Claude Code"
                            class="w-full rounded-2xl shadow-md"
                        />
                        <p class="text-warmgrey text-xs text-center mt-2">
                            "jebby walking through your first objective"
                        </p>
                    </div>
                </div>

                <div class="grid sm:grid-cols-2 gap-6 items-center">
                    <div>
                        <img
                            src="/imgs/jebby-prompt2.png"
                            alt="jebby breaking objectives into tasks in Claude Code"
                            class="w-full rounded-2xl shadow-md"
                        />
                        <p class="text-warmgrey text-xs text-center mt-2">
                            "9 tasks created across 3 objectives, just like that"
                        </p>
                    </div>
                    <div>
                        <p class="text-xs font-extrabold uppercase tracking-widest text-amber mb-4">
                            "✅ manage your tasks"
                        </p>
                        <div class="flex flex-col gap-3">
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "can you help me break my objectives down into tasks? for each task, let's set a deadline, priority, any relevant context, and tags if they make sense."
                                </p>
                            </div>
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "I just merged my pr. can you mark it as complete and tell me what's next on my list?"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="grid sm:grid-cols-2 gap-6 items-center">
                    <div>
                        <p class="text-xs font-extrabold uppercase tracking-widest text-amber mb-4">
                            "💬 check in with jebby"
                        </p>
                        <div class="flex flex-col gap-3">
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "can you ask jebby what I need to get done today? give me a briefing based on my priorities and any upcoming deadlines."
                                </p>
                            </div>
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "let's do a retro of my week. what did I get done, what slipped, and what should I carry forward?"
                                </p>
                            </div>
                            <div class="bg-warmgrey-pale dark:bg-surface-subtle border border-warmgrey-light dark:border-border-default rounded-2xl px-5 py-4">
                                <p class="text-warmblack dark:text-cream text-sm leading-relaxed">
                                    "what's coming up next week? help me plan ahead based on my tasks and objectives."
                                </p>
                            </div>
                        </div>
                    </div>
                    <div>
                        <img
                            src="/imgs/jebby-prompt3.png"
                            alt="jebby's daily briefing in Claude Code"
                            class="w-full rounded-2xl shadow-md"
                        />
                        <p class="text-warmgrey text-xs text-center mt-2">
                            "your daily briefing, ready before your first coffee"
                        </p>
                    </div>
                </div>

            </div>
        </section>
    }.into_any()
}
