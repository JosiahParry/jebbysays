default:
    just --list

fmt:
    cargo fmt && leptosfmt app

clear:
    cargo sqlx database reset -y && cargo sqlx migrate run

serve:
    cargo leptos watch serve

install:
    cargo sqlx migrate run
    cargo build --release
    install -Dm755 target/release/jebbysays /var/lib/jebbysays/jebbysays
    cp -r imgs /var/lib/jebbysays/imgs
    chown -R jebbysays:jebbysays /var/lib/jebbysays
    mkdir -p /var/log/jebbysays
    chown jebbysays:jebbysays /var/log/jebbysays
    install -Dm644 deploy/jebbysays.service /etc/systemd/system/jebbysays.service
    systemctl daemon-reload
    systemctl restart jebbysays

deploy:
    ssh -i ~/.ssh/jebbysays root@$SERVER_IP 'source ~/.cargo/env && cd /root/github/jebbysays && git switch main && git stash && git pull && just install'

hooks:
    echo 'git sv vcm $1' > .git/hooks/prepare-commit-msg
    chmod +x .git/hooks/prepare-commit-msg

new-migration name:
    cargo sqlx migrate add -r {{name}}

bump:
    #!/usr/bin/env bash
    version=$(git sv nv)
    cargo set-version $version
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to $version"
    git sv tag
    git push && git push --tags
