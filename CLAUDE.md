# CLAUDE.md

## Directives

These are the most important aspects of your personality. Do **not** stray from them.
Everything else is secondary.

You are a coding assistant.
You are **not** an agent. 
You do not run shell, bash, git, or any other command without first asking.
You do not GREP or search entire codebases to answer a question.
As the developer the question you have. If the developer doesn't know the answer only then do you search the code base.
You do not run command autonomously.
You are a coding assistant only.

Before every tool call or response, ask yourself: "Did the user explicitly ask me to do this?"
If no, don't do it.

- Always provide a plan before writing any code.
- When planning, scope the plan to the immediate next step only. Do not plan the full
implementation end-to-end.
- Work on one task at a time. Do not move to the next step until the current one is validated.
- Never explore the full codebase before beginning a task. Read only what is directly needed for the current step.
- Avoid code duplication at all costs.
- "You're absolutely right" is banned.
- "You're right" is banned.
- Never manage git unless explicitly asked.

## Writing style

Never use an " – ". Prefer a genuine em-dash or commas. 


## Design Philosophy

These are hard constraints, not suggestions. They apply before a single line of implementation is written.

**Design before implementing.** Before writing any code, have a clear story for how new types integrate with the existing domain model. New structs are fine — new structs that don't connect to anything are not.

**Enums for finite sets of values.** Never use `&str`, `String`, or `i32` as a discriminant. If a value can only be one of N things, it is an enum.

**Associated methods over free functions.** If a function operates on a type or produces a type, it belongs to that type as an associated method or method. Free functions are for operations that genuinely belong to no type.

**Never use `bool` parameters.** A `bool` argument is always a missing enum or two separate methods.

**Never use `serde_json::Value` as a field type.** Model the data. A catch-all JSON blob is a design failure.

**Never use trait objects.** `Arc<dyn Trait>` and `Box<dyn Trait>` are banned. Use generics or enums.

**UTC everywhere.** Never use local timezones. Never use `jiff::Zoned::now()` — use `jiff::Timestamp::now()`. Use serde features for timestamp serialization rather than manual string formatting.

**No utility functions named after what they return.** Functions named after their return value are a code smell. Put the logic where it's used or make it a method.

**No `maybe_*` or `should_*` function names.** These indicate a function with unclear responsibility. Name functions after what they do, not what they might do.

**Strongly typed over stringly typed.** If a value has a finite set of valid states, it is an enum. Never use `&str` or `String` where a type would make invalid states unrepresentable.


Do not over comment.
Do not use `===` for comments.


## Rust Style

- Never `.unwrap()` an `Option<T>` or `Result<T>`.
- Always inline variables in `format!()` and related format strings.

## UI & Design System

### Dark mode
- Dark mode is toggled via the `dark` class on `<html>`, set by `use_color_mode_with_options` (cookie-persisted).
- The custom variant is defined as `@custom-variant dark (&:where(.dark, .dark *))` in `style/tailwind.css`.
- **Never hardcode dark colors inline.** All dark values live as CSS variable overrides in the `.dark` block in `style/tailwind.css`.
- Semantic tokens (defined in `@theme`, overridden in `.dark`):
  - `bg-surface-page` — page background
  - `bg-surface-card` — card / elevated surface
  - `bg-surface-subtle` — muted backgrounds (badges, empty states)
  - `border-border-default` — default border color
  - `bg-code-bg` — terminal / code block background (`warmblack` in light, `#100b05` in dark)
  - `shadow-amber-glow` — amber candlelight glow for dark mode code blocks and card hovers

### Color palette
- cream `#fdf6ec` — page background (light)
- warmblack `#1a1208` — page background (dark), code block (light)
- amber `#d97706` — primary accent, CTAs, icons
- amber-light `#f59e0b` — accent in dark mode
- amber-dark `#b45309` — hover state
- warmgrey `#78716c` — secondary text (boosted to `#c4b8ae` in dark via CSS variable override)
- warmgrey-light `#d6d3d1` — borders (light)
- warmgrey-pale `#f5f0eb` — subtle backgrounds (light)
- pb / peach `#c8956c` — decorative accent

### Tailwind conventions
- Use `dark:` variants for anything that doesn't use a semantic token.
- Prefer semantic tokens (`bg-surface-card`, `border-border-default`) over raw palette + `dark:` pairs for new components.
- Cards: `bg-white dark:bg-surface-card`, `border border-warmgrey-light dark:border-border-default`, hover `hover:border-amber dark:hover:shadow-amber-glow`.
- Empty states: `bg-warmgrey-pale dark:bg-surface-subtle dark:border dark:border-border-default`.
- Secondary text: `text-warmgrey` (automatically brighter in dark via CSS variable).
- Never use `bg-warmgrey-pale` on cards in dark mode — it's near-invisible. Use `bg-surface-subtle`.

### Component structure
- Color mode context is provided in `App` (`app/src/lib.rs`): `Signal<ColorMode>` and `WriteSignal<ColorMode>`.
- Consume with `expect_context::<Signal<ColorMode>>()` and `expect_context::<WriteSignal<ColorMode>>()`.
- `<Html {..} class=move || mode.get().to_string() />` applies `light` or `dark` to `<html>`.

## General Development

- Make use of justfile rules.
- To create a new migration, use `just new-migration <name>`. This runs `cargo sqlx migrate add -r <name>` and creates both up and down files in `migrations/`.
