FROM rust:slim-bullseye

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
    build-essential \
    gcc \
    gcc-x86-64-linux-gnu \
    mingw-w64 \
    git \
    pkg-config \
    libx11-dev \
    libxtst-dev && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add i686-pc-windows-gnu
