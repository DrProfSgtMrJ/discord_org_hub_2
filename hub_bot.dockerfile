FROM rust:1.88-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY . .

RUN cargo install sea-orm-cli

RUN cargo build --release --bin hub_bot

FROM rust:1.88-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hub_bot ./hub_bot
COPY --from=builder /usr/local/cargo/bin/sea-orm-cli .
COPY migration ./migration

CMD ["sh", "-c", "./sea-orm-cli migrate up && ./hub_bot"]

