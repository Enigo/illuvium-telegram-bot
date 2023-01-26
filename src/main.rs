use dotenvy::dotenv;

use new_listing_api_reader::read_orders;

mod model;
mod new_listing_api_reader;
mod telegram_bot_sender;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    dotenv().expect(".env file should be present");

    read_orders();
}
