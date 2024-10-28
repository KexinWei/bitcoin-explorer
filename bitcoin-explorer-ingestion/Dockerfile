FROM debian:bookworm-slim as builder

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    ca-certificates \
    libpq-dev \
    pkg-config \
    libssl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq-dev \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/bitcoin-explorer-ingestion .

EXPOSE 3001
CMD ["./bitcoin-explorer-ingestion"]
