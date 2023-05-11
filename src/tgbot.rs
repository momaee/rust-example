use teloxide::prelude::*;

use crate::gpt;

pub async fn handler(bot: Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    let response = match msg.text() {
        Some(text) => {
            match gpt::chat(&text).await {
                Ok(response) => response,
                Err(err) => {
                    log::error!("Error: {}", err);
                    "Error".to_string()
                }
            }
        }
        None => "Send me plain text.".to_string(),
    };

    bot.send_message(msg.chat.id, response).await?;

    Ok(())
}