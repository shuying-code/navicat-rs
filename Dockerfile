# ---- build stage ----
FROM rust:1-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libxkbcommon-dev libgtk-3-dev libgl1-mesa-dev \
    libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev \
    libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
RUN cargo build --release

# ---- runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libxkbcommon0 libgl1 libgtk-3-0 \
    libx11-6 libxi6 libxrandr2 libxcursor1 libxinerama1 \
    libxcb1 libxcb-render0 libxcb-shape0 libxcb-xfixes0 \
    fonts-noto-cjk \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/navicat-rs /usr/local/bin/navicat-rs

# GUI app: needs an X11/Wayland display at runtime; --selftest works headless.
ENTRYPOINT ["/usr/local/bin/navicat-rs"]
CMD ["--selftest"]
