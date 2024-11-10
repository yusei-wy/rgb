FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    cmake \
    libsdl2-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add \
    clippy \
    rust-analysis \
    rust-analyzer \
    rust-docs \
    rust-src \
    rustfmt

USER vscode
