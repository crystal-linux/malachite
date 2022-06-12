pacman -Syu --noconfirm

pacman -S --noconfirm \
    curl \
    git \
    base-devel \
    lldb \
    rustup

rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy
rustup component add clippy --tolchain nightly

cargo install cargo-audit