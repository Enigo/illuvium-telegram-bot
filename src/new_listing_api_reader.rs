use chrono::{DateTime, Duration, Utc};
use futures::future::join_all;
use log::{error, info};
use tokio::task;

use crate::model::{asset::Asset, order::*};
use crate::telegram_bot_sender;

// max page_size=200
// cursor can be used to fetch the data from the beginning
const ORDERS_URL: &str = "https://api.x.immutable.com/v1/orders?status=active&sell_token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";
const ASSET_URL: &str = "https://api.x.immutable.com/v1/assets/0x9e0d99b864e1ac12565125c5a82b59adea5a09cd/";

#[tokio::main]
pub async fn read_orders() {
    let order_result = reqwest::get(ORDERS_URL)
        .await.unwrap().json::<Order>()
        .await;

    match order_result {
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
        let parse_result = reqwest::get(ASSET_URL.to_owned() + &result.sell.data.id)
            .await.unwrap().json::<Asset>()
            .await;

        match parse_result {
            Ok(asset) => {
                let buy = result.buy;
                telegram_bot_sender::send(asset.metadata, get_price(buy.data), buy.the_type)
                    .await;
            }
            Err(e) => {
                error!("Asset API response cannot be parsed! {}", e)
            }
        };
    }
}

fn get_price(data: BuyData) -> f32 {
    let index_of_comma = data.quantity.chars().count() - <i32 as TryInto<usize>>::try_into(data.decimals).unwrap();

    return match index_of_comma {
        0 => ("0.".to_owned() + &data.quantity).parse().unwrap(),
        _ => {
            let mut quantity_clone = data.quantity.clone();
            quantity_clone.insert(index_of_comma, '.');
            return quantity_clone.parse().unwrap();
        }
    };
}