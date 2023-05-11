use teloxide::{prelude::*};
use std::env;

mod tgbot;
mod gpt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let tel_api_key_dev = match env::var("TELEGRAM_API_KEY_DEV") {
        Ok(var) => {var},
        Err(e) => {
            log::error!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let bot = Bot::new(tel_api_key_dev);

    teloxide::repl(bot, tgbot::handler).await;
}
