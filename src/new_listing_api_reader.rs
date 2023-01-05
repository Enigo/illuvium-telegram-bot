use chrono::{DateTime, Duration, Utc};
use log::info;

use crate::model::{order::*, token::Token};
use crate::telegram_bot_sender;

// max page_size=200
// cursor can be used to fetch the data from the beginning
const ORDERS_URL: &str = "https://api.x.immutable.com/v1/orders?status=active&sell_token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd";
const TOKEN_URL: &str = "https://api.x.immutable.com/v1/assets/0x9e0d99b864e1ac12565125c5a82b59adea5a09cd/";

#[tokio::main]
pub async fn read_orders() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(ORDERS_URL)
        .await?
        .text()
        .await?;

    let result = serde_json::from_str::<Order>(&response);

    match result {
        Ok(order) => {
            return process_order(order)
                .await;
        }
        Err(e) => {
            panic!("Orders API response cannot be parsed! {}", e)
        }
    };
}

async fn process_order(order: Order) -> Result<(), Box<dyn std::error::Error>> {
    info!("Processing order response");
    for result in order.result {
        let timestamp = DateTime::parse_from_rfc3339(&result.timestamp).unwrap();
        let for_last_minutes = 1;
        let last_minute = Utc::now() - Duration::minutes(for_last_minutes);
        let is_after = timestamp > last_minute;

        if is_after {
            let response = reqwest::get(TOKEN_URL.to_owned() + &result.sell.data.id)
                .await?
                .text()
                .await?;
            let parse_result = serde_json::from_str::<Token>(&response);

            match parse_result {
                Ok(token) => {
                    let buy = result.buy;
                    telegram_bot_sender::send(token.metadata, get_price(buy.data), buy.the_type)
                        .await?;
                }
                Err(e) => {
                    panic!("Token API response cannot be parsed! {}", e)
                }
            };
        }
    }
    Ok(())
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