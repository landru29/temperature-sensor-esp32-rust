FROM rust:1.95.0-trixie

RUN apt update && apt install -y \
    build-essential \
    ca-certificates \
    dh-cargo \
    debhelper \
    devscripts \
    cmake \
    curl \
    git \
    pkg-config \
    unzip \
    vim \
    wget \
    zsh \
    usbutils \
    python3.13-venv \
    sudo

RUN groupadd -g 1000 vscode && \
    useradd -rm -d /home/vscode -s /bin/zsh -g root -G sudo,dialout -u 1000 vscode && \
    echo "vscode:insecure" | chpasswd

WORKDIR /workspace

USER vscode

ENV ESP_IDF_TOOLS_INSTALL_DIR=global

# Install OhMyZsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended

# Install esp32 tooling
RUN cargo install espup cargo-espflash ldproxy --locked && \
    espup install --extended-llvm

RUN rustup component add rust-src clippy rustfmt

RUN echo "source \$HOME/export-esp.sh" >> ~/.zshrc
