use rust_decimal::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::{env, fs};

use log::{debug, info, warn};
use rand::distributions::{Alphanumeric, DistString};
use resvg::usvg::Tree;
use resvg::{
    tiny_skia,
    usvg::{Options, TreeParsing},
    FitTo,
};
use teloxide::prelude::*;
use teloxide::types::InputFile;

use crate::model::asset::Asset;
use crate::model::order::TakerFees;

const USER_AGENT_VALUE: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
const ILLUVIDEX_ASSET_URL: &str = "https://illuvidex.illuvium.io/asset";

pub async fn send(asset: Asset, taker_fees: TakerFees) {
    let message = build_message(&asset, taker_fees);
    let bot = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN should be set"));
    let file_name = &format!(
        "/tmp/{}.png",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    );
    let path = Path::new(file_name);
    let chat_id = ChatId(
        env::var("CHAT_ID")
            .expect("CHAT_ID should be set")
            .parse()
            .expect("CHAT_ID should be a valid i64 value"),
    );

    match process_image(asset.metadata.image_url.as_str(), path).await {
        Ok(_) => {
            info!("Sending photo to telegram");
            match bot
                .send_photo(chat_id, InputFile::file(path))
                .caption(message)
                .await
            {
                Ok(message) => info!("Photo sent successfully {:?}", message.id),
                Err(e) => warn!("Photo wasn't sent because of: {}", e),
            };

            match fs::remove_file(path) {
                Err(e) => {
                    debug!("Failed to delete the file {} {}", file_name, e)
                }
                _ => {}
            };
        }
        Err(e) => {
            warn!(
                "Couldn't generate image because of `{}`, falling back to text message",
                e
            );
            match bot.send_message(chat_id, message).await {
                Ok(message) => info!("Text message sent successfully {:?}", message.id),
                Err(e) => warn!("Text message wasn't sent because of: {}", e),
            };
        }
    }
}

fn build_message(asset: &Asset, taker_fees: TakerFees) -> String {
    let metadata = &asset.metadata;
    let price = get_price(&taker_fees);
    let symbol = taker_fees.symbol;

    let (tier, name, landmark, hydrogen, carbon, silicon, solon, crypton, hyperion, url) = (
        &metadata.tier,
        &metadata.name,
        &metadata.landmark,
        &metadata.hydrogen,
        &metadata.carbon,
        &metadata.silicon,
        &metadata.solon,
        &metadata.crypton,
        &metadata.hyperion,
        format!(
            "{}/{}/{}",
            ILLUVIDEX_ASSET_URL, asset.token_address, asset.token_id
        ),
    );

    format!(
        "T{tier} {name}\nprice: {price}{symbol}\nlandmark: {landmark}\
    \nhydrogen: {hydrogen} carbon: {carbon} silicon: {silicon}\
    \nsolon: {solon} crypton: {crypton} hyperion: {hyperion}\
    \n{url}"
    )
}

pub fn get_price(taker_fees: &TakerFees) -> f32 {
    let wei_value = Decimal::from_str(&taker_fees.quantity_with_fees).unwrap();
    let ether_value = wei_value / Decimal::new(10i64.pow(taker_fees.decimals as u32), 0);
    ether_value.to_f32().unwrap()
}

async fn process_image(
    image_url: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let image_url = reqwest::Url::parse(image_url)?;
    let client = reqwest::Client::new();
    let response = client
        .get(image_url)
        .header(reqwest::header::USER_AGENT, USER_AGENT_VALUE)
        .send()
        .await?;
    let status_code = response.status();

    if status_code == reqwest::StatusCode::OK {
        let img_text = &response.text().await?;
        generate_image(img_text, path)?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            ErrorKind::Unsupported,
            "Status code wasn't OK but ".to_owned() + status_code.as_str(),
        )))
    }
}

fn generate_image(
    svg_text: &String,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tree: Tree = TreeParsing::from_str(svg_text, &Options::default())?;

    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("Couldn't create pixmap")?;
    resvg::render(
        &tree,
        FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .ok_or("Couldn't render pixmap")?;

    pixmap.save_png(path)?;

    Ok(())
}
