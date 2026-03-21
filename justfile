default:
    just --list

fmt:
    cargo fmt

clear:
    cargo sqlx database reset -y && cargo sqlx migrate run

serve:
    cargo run -- serve

install:
    cargo build --release
    sudo install -Dm755 target/release/jebbysays /var/lib/jebbysays/jebbysays
    sudo install -Dm644 deploy/jebbysays.service /etc/systemd/system/jebbysays.service
    sudo systemctl daemon-reload

new-migration name:
    cargo sqlx migrate add -r {{name}}
