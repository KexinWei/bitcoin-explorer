use tokio_postgres::Client;
use std::sync::Arc;
use serde::Deserialize;
use chrono::{NaiveDateTime, Utc};
use reqwest;

#[derive(Deserialize)]
pub struct MarketData {
    pub prices: Vec<[f64; 2]>,
    pub total_volumes: Vec<[f64; 2]>,
}

#[derive(Deserialize)]
pub struct ChartData {
    pub values: Vec<DataPoint>,
}

#[derive(Deserialize)]
pub struct DataPoint {
    pub x: i64,
    pub y: f64,
}

pub async fn fetch_historical_market_data(days: &str) -> Result<MarketData, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days={}", days);
    let response = reqwest::get(&url).await?;
    let data: MarketData = response.json().await?;
    Ok(data)
}

pub async fn fetch_historical_network_data(timespan: &str, chart_type: &str) -> Result<ChartData, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://api.blockchain.info/charts/{}?timespan={}&format=json&cors=true", chart_type, timespan);
    let response = reqwest::get(&url).await?;
    let data: ChartData = response.json().await?;
    Ok(data)
}

pub async fn insert_historical_market_data(client: Arc<Client>, market_data: MarketData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for (price_point, volume_point) in market_data.prices.iter().zip(market_data.total_volumes.iter()) {
        let timestamp_ms = price_point[0] as i64;
        let price = price_point[1];
        let volume = volume_point[1];

        let timestamp = NaiveDateTime::from_timestamp(timestamp_ms / 1000, 0);

        client.execute(
            "INSERT INTO market_data (timestamp, price_usd, volume_usd) VALUES ($1, $2, $3)",
            &[&timestamp, &price, &volume],
        ).await?;
    }
    Ok(())
}

pub async fn insert_historical_network_data(client: Arc<Client>, hash_rate_data: ChartData, difficulty_data: ChartData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for (hash_rate_point, difficulty_point) in hash_rate_data.values.iter().zip(difficulty_data.values.iter()) {
        let timestamp = NaiveDateTime::from_timestamp(hash_rate_point.x, 0);
        let hash_rate = hash_rate_point.y;
        let difficulty = difficulty_point.y;

        client.execute(
            "INSERT INTO network_stats (timestamp, hash_rate, difficulty) VALUES ($1, $2, $3)",
            &[&timestamp, &hash_rate, &difficulty],
        ).await?;
    }
    Ok(())
}

pub async fn fetch_latest_market_data() -> Result<(NaiveDateTime, f64, f64), Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=1";
    let response = reqwest::get(url).await?;
    let data: MarketData = response.json().await?;
    if let Some(price_point) = data.prices.last() {
        if let Some(volume_point) = data.total_volumes.last() {
            let timestamp_ms = price_point[0] as i64;
            let price = price_point[1];
            let volume = volume_point[1];
            let timestamp = NaiveDateTime::from_timestamp(timestamp_ms / 1000, 0);
            return Ok((timestamp, price, volume));
        }
    }
    Err("No market data available".into())
}

pub async fn fetch_latest_network_data() -> Result<(NaiveDateTime, f64, f64), Box<dyn std::error::Error + Send + Sync>> {
    let hash_rate_url = "https://api.blockchain.info/charts/hash-rate?timespan=1days&format=json&cors=true";
    let difficulty_url = "https://api.blockchain.info/charts/difficulty?timespan=1days&format=json&cors=true";

    let hash_rate_response = reqwest::get(hash_rate_url).await?;
    let difficulty_response = reqwest::get(difficulty_url).await?;

    let hash_rate_data: ChartData = hash_rate_response.json().await?;
    let difficulty_data: ChartData = difficulty_response.json().await?;

    if let Some(hash_rate_point) = hash_rate_data.values.last() {
        if let Some(difficulty_point) = difficulty_data.values.last() {
            let timestamp = NaiveDateTime::from_timestamp(hash_rate_point.x, 0);
            let hash_rate = hash_rate_point.y;
            let difficulty = difficulty_point.y;
            return Ok((timestamp, hash_rate, difficulty));
        }
    }
    Err("No network data available".into())
}

pub async fn fetch_latest_block_height() -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let api_url = "https://blockstream.info/api/blocks/tip/height";
    let response = reqwest::get(api_url).await?;
    let block_height: i64 = response.text().await?.trim().parse()?;
    Ok(block_height)
}

pub async fn fetch_block_details(height: i64) -> Result<(String, serde_json::Value), Box<dyn std::error::Error + Send + Sync>> {
    let api_url = format!("https://blockstream.info/api/block-height/{}", height);
    let response = reqwest::get(&api_url).await?;
    let block_hash = response.text().await?.trim().to_string();

    let api_url = format!("https://blockstream.info/api/block/{}", block_hash);
    let response = reqwest::get(&api_url).await?;
    let block_details: serde_json::Value = response.json().await?;

    Ok((block_hash, block_details))
}

pub async fn insert_latest_block(client: Arc<Client>, block_height: i64, block_hash: String, block_details: serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = block_details["timestamp"].as_i64().unwrap_or(0);
    let tx_count = block_details["tx_count"].as_i64().unwrap_or(0);
    let size = block_details["size"].as_i64().unwrap_or(0);
    let weight = block_details["weight"].as_i64().unwrap_or(0);

    let timestamp = NaiveDateTime::from_timestamp(timestamp, 0);
    let tx_count_i32 = tx_count as i32;

    client.execute(
        "INSERT INTO blocks (block_hash, height, timestamp, tx_count, size, weight)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (height) DO UPDATE SET block_hash = $1, timestamp = $3, tx_count = $4, size = $5, weight = $6",
        &[&block_hash, &block_height, &timestamp, &tx_count_i32, &size, &weight],
    ).await?;

    Ok(())
}

pub async fn start_ingestion_loop(client: Arc<Client>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market_data_rows = client.query("SELECT COUNT(*) FROM market_data", &[]).await?;
    let market_data_count: i64 = market_data_rows[0].get(0);

    if market_data_count == 0 {
        let market_data = fetch_historical_market_data("365").await?;
        insert_historical_market_data(client.clone(), market_data).await?;
        println!("Inserted historical market data.");
    }

    let network_data_rows = client.query("SELECT COUNT(*) FROM network_stats", &[]).await?;
    let network_data_count: i64 = network_data_rows[0].get(0);

    if network_data_count == 0 {
        let hash_rate_data = fetch_historical_network_data("1year", "hash-rate").await?;
        let difficulty_data = fetch_historical_network_data("1year", "difficulty").await?;
        insert_historical_network_data(client.clone(), hash_rate_data, difficulty_data).await?;
        println!("Inserted historical network data.");
    }

    let mut market_data_interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
    let mut network_data_interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
    let mut block_data_interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

    loop {
        tokio::select! {
            _ = market_data_interval.tick() => {
                match fetch_latest_market_data().await {
                    Ok((timestamp, price, volume)) => {
                        client.execute(
                            "INSERT INTO market_data (timestamp, price_usd, volume_usd) VALUES ($1, $2, $3)",
                            &[&timestamp, &price, &volume],
                        ).await?;
                        println!("Inserted latest market data: ${}", price);
                    }
                    Err(e) => eprintln!("Fetching latest market data error: {:?}", e),
                }
            }
            _ = network_data_interval.tick() => {
                match fetch_latest_network_data().await {
                    Ok((timestamp, hash_rate, difficulty)) => {
                        client.execute(
                            "INSERT INTO network_stats (timestamp, hash_rate, difficulty) VALUES ($1, $2, $3)",
                            &[&timestamp, &hash_rate, &difficulty],
                        ).await?;
                        println!("Inserted latest network data: Hash Rate {}, Difficulty {}", hash_rate, difficulty);
                    }
                    Err(e) => eprintln!("Fetching latest network data error: {:?}", e),
                }
            }
            _ = block_data_interval.tick() => {
                match fetch_latest_block_height().await {
                    Ok(block_height) => {
                        let rows = client.query("SELECT height FROM blocks ORDER BY height DESC LIMIT 1", &[]).await?;
                        let mut latest_height: i64 = 0;
                        if !rows.is_empty() {
                            latest_height = rows[0].get(0);
                        }

                        if block_height > latest_height {
                            match fetch_block_details(block_height).await {
                                Ok((block_hash, block_details)) => {
                                    insert_latest_block(client.clone(), block_height, block_hash.clone(), block_details.clone()).await?;
                                    println!("Inserted latest block data: Height {}", block_height);
                                }
                                Err(e) => eprintln!("Fetching block details error: {:?}", e),
                            }
                        } else {
                            println!("No new block. Current height: {}", block_height);
                        }
                    }
                    Err(e) => eprintln!("Fetching latest block height error: {:?}", e),
                }
            }
        }
    }
}
