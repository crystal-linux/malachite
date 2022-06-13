pacman -Syu --noconfirm

pacman -S --noconfirm \
    curl \
    git \
    base-devel \
    lldb \
    rustup

rustup install nightly
rustup component add rustfmt
rustup component add clippy

cargo install cargo-audit
