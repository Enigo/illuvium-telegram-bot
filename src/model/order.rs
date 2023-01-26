use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Order {
    pub result: Vec<TheResult>,
}

#[derive(Deserialize, Debug)]
pub struct TheResult {
    pub timestamp: String,
    pub sell: Sell,
    pub buy: Buy,
}

#[derive(Deserialize, Debug)]
pub struct Sell {
    pub data: SellData,
}

#[derive(Deserialize, Debug)]
pub struct Buy {
    pub data: BuyData,
    #[serde(rename = "type")]
    pub the_type: String,
}

#[derive(Deserialize, Debug)]
pub struct BuyData {
    pub decimals: i32,
    pub quantity: String,
}

#[derive(Deserialize, Debug)]
pub struct SellData {
    pub id: String,
}
