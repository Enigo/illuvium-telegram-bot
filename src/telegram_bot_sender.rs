use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::info;
use reqwest::StatusCode;
use resvg::{tiny_skia, usvg::{Options, Tree, FitTo}};
use teloxide::prelude::*;
use teloxide::types::InputFile;

use crate::model::token::Metadata;

const USER_AGENT_VALUE: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
const BOT_TOKEN: &str = "<>"; // insert correct one
const CHAT_ID: i64 = 123; // insert correct one

pub async fn send(metadata: Metadata, price: f32, buy_type: String) -> Result<(), Box<dyn std::error::Error>> {
    let token_type = HashMap::from([
        (String::from("ERC20"), String::from("USDC"))
    ]);
    let token = token_type.get(&buy_type).map_or("ETH", String::as_str);

    let bot = Bot::new(BOT_TOKEN);

    let image_url = reqwest::Url::parse(&metadata.image_url).unwrap();
    let client = reqwest::Client::new();
    let response = client.get(image_url)
        .header(reqwest::header::USER_AGENT, USER_AGENT_VALUE)
        .send().await?;
    let status_code = response.status();

    if status_code != StatusCode::OK {
        panic!("Response code wasn't OK but {}", status_code)
    }

    let img_text = &response.text().await?;
    let path = &Path::new("/tmp/image.png");

    generate_image(&img_text, path);
    let message = build_message(&metadata, &price, token);

    info!("Sending photo to telegram");
    bot.send_photo(ChatId(CHAT_ID), InputFile::file(path)).caption(message).await?;
    fs::remove_file(path)?;

    Ok(())
}

fn build_message(metadata: &Metadata, price: &f32, token: &str) -> String {
    "T".to_owned() + &metadata.tier.to_string() + " " + &metadata.name
        + " price: " + &price.to_string() + token + " landmark: " + &metadata.landmark
        + " hydrogen:" + &metadata.hydrogen.to_string() + " carbon:" + &metadata.carbon.to_string()
        + " silicon:" + &metadata.silicon.to_string() + " solon:" + &metadata.solon.to_string()
        + " crypton:" + &metadata.crypton.to_string() + " hyperion:" + &metadata.hyperion.to_string()
}

fn generate_image(svg_text: &String, path: &Path) {
    let tree = Tree::from_str(svg_text, &Options::default()).unwrap();

    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    ).unwrap();

    pixmap.save_png(path).unwrap();
}