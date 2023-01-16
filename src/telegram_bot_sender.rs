use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use log::{debug, info, warn};
use rand::distributions::{Alphanumeric, DistString};
use resvg::{tiny_skia, usvg::{FitTo, Options, Tree}};
use teloxide::prelude::*;
use teloxide::types::InputFile;

use crate::model::asset::Metadata;
use crate::model::order::{Buy, BuyData};

const USER_AGENT_VALUE: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
const BOT_TOKEN: &str = "<>"; // insert correct one
const CHAT_ID: i64 = 123; // insert correct one

pub async fn send(metadata: Metadata, buy: Buy) {
    let message = build_message(&metadata, buy);
    let bot = Bot::new(BOT_TOKEN);
    let file_name = &format!("/tmp/{}.png", Alphanumeric.sample_string(&mut rand::thread_rng(), 16));
    let path = Path::new(file_name);

    match process_image(&metadata.image_url, path).await {
        Ok(_) => {
            info!("Sending photo to telegram");
            match bot.send_photo(ChatId(CHAT_ID), InputFile::file(path)).caption(message).await
            {
                Ok(message) => info!("Photo sent successfully {:?}", message.id),
                Err(e) => warn!("Photo wasn't sent because of: {}", e)
            };

            match fs::remove_file(path) {
                Err(e) => {
                    debug!("Failed to delete the file {} {}", file_name, e)
                }
                _ => {}
            };
        }
        Err(e) => {
            warn!("Couldn't generate image because of `{}`, falling back to text message", e);
            match bot.send_message(ChatId(CHAT_ID), message).await
            {
                Ok(message) => info!("Text message sent successfully {:?}", message.id),
                Err(e) => warn!("Text message wasn't sent because of: {}", e)
            };
        }
    }
}

fn build_message(metadata: &Metadata, buy: Buy) -> String {
    let price = get_price(buy.data);
    let token_type = HashMap::from([
        (String::from("ERC20"), String::from("USDC"))
    ]);
    let token = token_type.get(&buy.the_type).map_or("ETH", String::as_str);

    "T".to_owned() + &metadata.tier.to_string() + " " + &metadata.name
        + " price: " + &price.to_string() + token + " landmark: " + &metadata.landmark
        + " hydrogen:" + &metadata.hydrogen.to_string() + " carbon:" + &metadata.carbon.to_string()
        + " silicon:" + &metadata.silicon.to_string() + " solon:" + &metadata.solon.to_string()
        + " crypton:" + &metadata.crypton.to_string() + " hyperion:" + &metadata.hyperion.to_string()
}

fn get_price(data: BuyData) -> f32 {
    let index_of_comma = data.quantity.chars().count() - <i32 as TryInto<usize>>::try_into(data.decimals).unwrap();

    return match index_of_comma {
        0 => ("0.".to_owned() + &data.quantity).parse().unwrap(),
        _ => {
            let mut quantity_clone = data.quantity.clone();
            quantity_clone.insert(index_of_comma, '.');
            quantity_clone.parse().unwrap()
        }
    };
}

async fn process_image(image_url: &String, path: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let image_url = reqwest::Url::parse(&image_url)?;
    let client = reqwest::Client::new();
    let response = client.get(image_url)
        .header(reqwest::header::USER_AGENT, USER_AGENT_VALUE)
        .send().await?;
    let status_code = response.status();

    if status_code == reqwest::StatusCode::OK {
        let img_text = &response.text().await?;
        generate_image(img_text, path)?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(ErrorKind::Unsupported, "Status code wasn't OK but ".to_owned() + status_code.as_str())))
    }
}

fn generate_image(svg_text: &String, path: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tree = Tree::from_str(svg_text, &Options::default())?;

    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).ok_or("Couldn't create pixmap")?;
    resvg::render(
        &tree,
        FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    ).ok_or("Couldn't render pixmap")?;

    pixmap.save_png(path)?;

    Ok(())
}