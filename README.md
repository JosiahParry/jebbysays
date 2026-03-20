# Chief - your personal Chief of Staff

> "Don't sweat the small stuff"

`chief` is your very own personal chief of staff designed to keep track of all of your 💩.

- Do you, like me, have crippling ADHD?
- Do you need an accountability buddy?
- Do you struggle to keep track of your own todo list?

Just talk to your CoS and they'll help you prioritize, shift focus, or triage urgent tasks as they arise.

Inspired by my own attempt at using Claude + Notion MCP as a personal assistant. It worked exceptionally well — until it didn't. The Notion MCP server [has some challenges](https://github.com/makenotion/notion-mcp-server/issues/47#issuecomment-3739384218).

## Installation

```bash
cargo install --git https://github.com/josiahparry/chief
```

## Getting started

Chief runs as a local HTTP server backed by a SQLite database.

```bash
chief
```

By default the database lives at `~/.local/share/chief/todo.sqlite3`. Override with the `CHIEF_PATH` environment variable.

### Connect via Claude Code

Create an MCP connection for your user. 

```bash
claude mcp add chief --scope user --transport http http://localhost:24433/mcp
```

## What it does

Chief gives your AI a structured view of your work through two concepts:

**Objectives** — the goals and projects that matter to you. Each has a priority so your AI knows what to focus on.

**Tasks** — the concrete work that moves your objectives forward. Tasks have priorities, deadlines, tags, and belong to an objective.

## Resources

| URI | Description |
|---|---|
| `tasks://all` | All tasks |
| `tasks://incomplete` | Incomplete tasks |
| `tasks://completed` | Completed tasks |
| `objectives://all` | All objectives |
| `task://{id}` | A specific task |
| `objective://{id}` | A specific objective |

## Tools

| Tool | Description |
|---|---|
| `add_task` | Create a new task |
| `modify_task` | Update fields on a task |
| `complete_task` | Mark a task as done |
| `delete_task` | Delete a task |
| `add_objective` | Create a new objective |
| `modify_objective` | Update an objective |
| `delete_objective` | Delete an objective |
