default:
    just --list

fmt:
    cargo fmt

clear:
    cargo sqlx database reset -y && cargo sqlx migrate run

serve:
    cargo run -- serve

new-migration name:
    cargo sqlx migrate add -r {{name}}
