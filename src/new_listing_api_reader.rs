use chrono::{DateTime, Duration, Utc};
use futures::future::join_all;
use log::{error, info};
use serde::de::DeserializeOwned;
use tokio::task;

use crate::model::{asset::Asset, order::*};
use crate::telegram_bot_sender;

const ORDERS_URL: &str = "https://api.x.immutable.com/v1/orders?status=active&sell_token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";
const ASSET_URL: &str = "https://api.x.immutable.com/v1/assets/0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";

#[tokio::main]
pub async fn read_orders() {
    let response = fetch_api_response::<Order>(ORDERS_URL).await;
    match response {
        Ok(order) => {
            info!("Processing order response");
            let mut futures = vec![];
            for result in order.result {
                let future = task::spawn(process_order(result));
                futures.push(future);
            }

            join_all(futures).await;
        }
        Err(e) => {
            error!("Orders API response cannot be parsed! {}", e)
        }
    };
}

async fn process_order(result: TheResult) {
    let timestamp = DateTime::parse_from_rfc3339(&result.timestamp).unwrap();
    let for_last_minutes = 1;
    let last_minute = Utc::now() - Duration::minutes(for_last_minutes);
    let is_after = timestamp > last_minute;

    if is_after {
        info!("Newly listed land detected");
        let response = fetch_api_response::<Asset>(format!("{}/{}", ASSET_URL, &result.sell.data.id).as_str()).await;
        match response {
            Ok(asset) => {
                let buy = result.buy;
                telegram_bot_sender::send(&asset, buy).await;
            }
            Err(e) => {
                error!("Asset API response cannot be parsed! {}", e)
            }
        };
    }
}

async fn fetch_api_response<T: DeserializeOwned>(endpoint: &str) -> reqwest::Result<T> {
    let result = reqwest::get(endpoint).await?.json::<T>().await?;
    return Ok(result);
}
