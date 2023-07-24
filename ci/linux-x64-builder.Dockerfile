FROM rust:slim-bullseye

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
    build-essential \
    gcc \
    mingw-w64 \
    gcc-multilib \
    git \
    pkg-config \
    libx11-dev \
    libxtst-dev && \
    rustup target add x86_64-unknown-linux-gnu
