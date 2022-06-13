FROM crystallinux/crystal:crystal

WORKDIR /home/
COPY . .

RUN useradd -m vscode

RUN bash ./setup.sh

USER vscode
RUN bash ./rust.sh

ENV PATH="/root/.cargo/bin:$PATH"
