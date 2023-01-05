use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Token {
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub tier: i64,
    pub solon: i64,
    pub carbon: i64,
    pub crypton: i64,
    pub silicon: i64,
    pub hydrogen: i64,
    pub hyperion: i64,
    pub landmark: String,
    pub image_url: String,
}