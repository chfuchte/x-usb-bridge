FROM rust:1-bookworm AS builder

RUN dpkg --add-architecture arm64 \
    && apt-get update \
    && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    libasound2-dev:arm64 \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
ENV PKG_CONFIG_SYSROOT_DIR=/

WORKDIR /app

COPY . .

RUN cargo build --release --target aarch64-unknown-linux-gnu


FROM scratch AS output

COPY --from=builder \
    /app/target/aarch64-unknown-linux-gnu/release/x-usb-bridge \
    /x-usb-bridge
