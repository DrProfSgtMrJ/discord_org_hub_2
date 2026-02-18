FROM rust:1.88-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY . .

RUN cargo build --release --bin hub_bot

FROM rust:1.88-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hub_bot ./hub_bot

CMD ["./hub_bot"]

