use teloxide::{prelude::*};
use std::{env,thread};

mod tgbot;
mod gpt;
mod server;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    log::info!("Starting application...");

    thread::spawn(|| {
        match server::serve() {
           Ok(_) => {},
            Err(e) => {
            log::error!("Failed to start server: {}", e);
            return;
            }
        };
    });
    
    let tel_api_key_dev = match env::var("TELEGRAM_API_KEY_DEV") {
        Ok(var) => {var},
        Err(e) => {
            log::error!("Failed to get TELEGRAM_API_KEY_DEV: {}", e);
            return;
        }
    };

    let bot = Bot::new(tel_api_key_dev);

    log::info!("Starting bot...");
    teloxide::repl(bot, tgbot::handler).await;
}