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
    install -Dm755 target/release/jebbysays /var/lib/jebbysays/jebbysays
    chown -R jebbysays:jebbysays /var/lib/jebbysays
    mkdir -p /var/log/jebbysays
    chown jebbysays:jebbysays /var/log/jebbysays
    install -Dm644 deploy/jebbysays.service /etc/systemd/system/jebbysays.service
    systemctl daemon-reload
    systemctl restart jebbysays

new-migration name:
    cargo sqlx migrate add -r {{name}}
