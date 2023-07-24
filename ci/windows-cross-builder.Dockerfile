FROM rust:slim-bullseye

RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends \
    build-essential \
    mingw-w64 \
    git && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add i686-pc-windows-gnu
