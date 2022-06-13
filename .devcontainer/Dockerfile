FROM crystallinux/crystal:crystal

WORKDIR /home/
COPY . .

RUN bash ./setup.sh

RUN useradd -m -G wheel vscode

USER vscode
RUN bash ./rust.sh

ENV PATH="/root/.cargo/bin:$PATH"
