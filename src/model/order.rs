use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Order {
    pub result: Vec<TheResult>,
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub timestamp: String,
    pub sell: Sell,
    pub taker_fees: TakerFees,
}

#[derive(Deserialize, Debug)]
pub struct Sell {
    pub data: SellData,
}

#[derive(Deserialize, Debug)]
pub struct SellData {
    pub token_id: String,
}

#[derive(Deserialize, Debug)]
pub struct TakerFees {
    pub symbol: String,
    pub decimals: i32,
    pub quantity_with_fees: String,
}
