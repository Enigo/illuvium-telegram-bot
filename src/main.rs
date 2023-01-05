extern crate core;

use new_listing_api_reader::read_orders;

mod new_listing_api_reader;
mod telegram_bot_sender;
mod model;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    match read_orders() {
        Err(e) => println!("{:?}", e),
        _ => ()
    };
}
