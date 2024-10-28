use warp::Filter;
use std::sync::Arc;
use tokio_postgres::Client;
use serde::Serialize;
use serde_json::json;
use warp::http::StatusCode;
use warp::reply::{Reply, Response};

#[derive(Serialize)]
pub struct MarketDataResponse {
    pub timestamp: String,
    pub price_usd: f64,
    pub volume_usd: f64,
}

#[derive(Serialize)]
pub struct NetworkDataResponse {
    pub timestamp: String,
    pub hash_rate: f64,
    pub difficulty: f64,
}

#[derive(Serialize)]
pub struct LatestBlockResponse {
    pub block_hash: String,
    pub height: i64,
    pub timestamp: String,
    pub tx_count: i32,
    pub size: i64,
    pub weight: i64,
}

pub async fn get_market_data(client: Arc<Client>) -> Result<Response, warp::Rejection> {
    match client
        .query("SELECT to_char(timestamp, 'YYYY-MM-DD HH24:MI:SS') as timestamp_str, price_usd, volume_usd FROM market_data ORDER BY timestamp ASC", &[])
        .await
    {
        Ok(rows) => {
            let market_data: Vec<MarketDataResponse> = rows.iter().map(|row| {
                MarketDataResponse {
                    timestamp: row.get(0),
                    price_usd: row.get(1),
                    volume_usd: row.get(2),
                }
            }).collect();

            let success_reply = warp::reply::json(&market_data);
            Ok(success_reply.into_response())
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            let error_message = json!({ "error": format!("Database query error: {:?}", e) });
            let error_reply = warp::reply::with_status(
                warp::reply::json(&error_message),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Ok(error_reply.into_response())
        }
    }
}

pub async fn get_network_data(client: Arc<Client>) -> Result<Response, warp::Rejection> {
    match client
        .query("SELECT to_char(timestamp, 'YYYY-MM-DD HH24:MI:SS') as timestamp_str, hash_rate, difficulty FROM network_stats ORDER BY timestamp ASC", &[])
        .await
    {
        Ok(rows) => {
            let network_data: Vec<NetworkDataResponse> = rows.iter().map(|row| {
                NetworkDataResponse {
                    timestamp: row.get(0),
                    hash_rate: row.get(1),
                    difficulty: row.get(2),
                }
            }).collect();

            let success_reply = warp::reply::json(&network_data);
            Ok(success_reply.into_response())
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            let error_message = json!({ "error": format!("Database query error: {:?}", e) });
            let error_reply = warp::reply::with_status(
                warp::reply::json(&error_message),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Ok(error_reply.into_response())
        }
    }
}

pub async fn get_latest_block(client: Arc<Client>) -> Result<Response, warp::Rejection> {
    match client
        .query(
            "SELECT block_hash, height, to_char(timestamp, 'YYYY-MM-DD HH24:MI:SS') as timestamp_str, tx_count, size, weight FROM blocks ORDER BY height DESC LIMIT 1",
            &[],
        )
        .await
    {
        Ok(rows) => {
            if !rows.is_empty() {
                let block_hash: String = rows[0].get(0);
                let height: i64 = rows[0].get(1);
                let timestamp: String = rows[0].get(2);
                let tx_count: i32 = rows[0].get(3);
                let size: i64 = rows[0].get(4);
                let weight: i64 = rows[0].get(5);

                let success_reply = warp::reply::json(&LatestBlockResponse {
                    block_hash,
                    height,
                    timestamp,
                    tx_count,
                    size,
                    weight,
                });
                Ok(success_reply.into_response())
            } else {
                eprintln!("No data found in blocks table.");
                let error_message = json!({ "error": "No data found in blocks table." });
                let error_reply = warp::reply::with_status(
                    warp::reply::json(&error_message),
                    StatusCode::NOT_FOUND,
                );
                Ok(error_reply.into_response())
            }
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            let error_message = json!({ "error": format!("Database query error: {:?}", e) });
            let error_reply = warp::reply::with_status(
                warp::reply::json(&error_message),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            Ok(error_reply.into_response())
        }
    }
}

pub async fn start_server(client: Arc<Client>) {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"])
        .allow_header("content-type");

    let client_filter = warp::any().map(move || Arc::clone(&client));

    let market_data_route = warp::path("market-data")
        .and(warp::get())
        .and(client_filter.clone())
        .and_then(get_market_data)
        .with(cors.clone());

    let network_data_route = warp::path("network-data")
        .and(warp::get())
        .and(client_filter.clone())
        .and_then(get_network_data)
        .with(cors.clone());

    let latest_block_route = warp::path("latest-block")
        .and(warp::get())
        .and(client_filter.clone())
        .and_then(get_latest_block)
        .with(cors.clone());

    let routes = market_data_route
        .or(network_data_route)
        .or(latest_block_route);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3001))
        .await;
}
