pacman -Syu --noconfirm

pacman -S --noconfirm \
    curl \
    git \
    base-devel \
    lldb \
    rustup

echo "%wheel ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers