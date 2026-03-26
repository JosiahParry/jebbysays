# CHANGELOG

## Development

## 0.2.6

### Features

- Migrated to Cargo workspace (`app`, `frontend`, `jebbysays-core`)
- Full web UI built with Leptos: landing page, user dashboard for objectives and tasks
- PKCE OAuth flow with JWKS validation and cookie-persisted sessions
- Dark mode with cookie-persisted preference
- Embedded static asset serving via `rust-embed`

### Fixes

- Improved schemars descriptions for more reliable MCP agent interaction
- `objective_id` is now required on `add_task`
 
## 0.1.1

### Features

- Task dependencies (`depends_on`)

## 0.1.0

### Features

- Initial release
