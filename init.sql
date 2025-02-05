CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    block_hash VARCHAR(64) NOT NULL UNIQUE,
    height BIGINT NOT NULL UNIQUE,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    tx_count INTEGER NOT NULL,
    size BIGINT,
    weight BIGINT
);

CREATE TABLE IF NOT EXISTS market_data (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    price_usd DOUBLE PRECISION NOT NULL,
    volume_usd DOUBLE PRECISION NOT NULL
);

CREATE TABLE IF NOT EXISTS network_stats (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    hash_rate DOUBLE PRECISION NOT NULL,
    difficulty DOUBLE PRECISION NOT NULL
);
