FROM crystallinux/crystal:crystal

WORKDIR /home/
COPY . .

RUN bash ./setup.sh

RUN useradd -m -G wheel vscode

USER vscode
RUN rustup install nightly

RUN sudo pacman --noconfirm -S pfetch
RUN echo "alias clear=\"clear; pfetch\"" >> /home/vscode/.bashrc
RUN echo "pfetch" >> /home/vscode/.bashrc

ENV PS1="\[\033[38;5;8m\]\T\[$(tput sgr0)\] \[$(tput sgr0)\]\[\033[38;5;13m\]\W\[$(tput sgr0)\] \$(git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/(\1)/') \[$(tput sgr0)\]\[\033[38;5;7m\][\$?]\[$(tput sgr0)\]\n\[$(tput sgr0)\]\[\033[38;5;5m\]\\$\[$(tput sgr0)\] \[$(tput sgr0)\]"
