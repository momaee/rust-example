use teloxide::{prelude::*};
use std::{
    env,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

mod tgbot;
mod gpt;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    log::info!("Starting application...");

    let port = match env::var("PORT") {
        Ok(var) => {var},
        Err(e) => {
            log::debug!("Error: {}", e);
            log::debug!("Using default port 8080");
            String::from("8080")
        }
    };

    let listener = match TcpListener::bind("0.0.0.0:".to_owned() + &port) {
        Ok(listener) => {listener},
        Err(e) => {
            log::error!("Error: {}", e);
            std::process::exit(1);
        }
    };

    thread::spawn(move || {
        log::info!("Listening on port {}", port);
        listen(listener);
    }); 

    let tel_api_key_dev = match env::var("TELEGRAM_API_KEY_DEV") {
        Ok(var) => {var},
        Err(e) => {
            log::error!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let bot = Bot::new(tel_api_key_dev);

    log::info!("Starting bot...");
    teloxide::repl(bot, tgbot::handler).await;
}

fn listen(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream); 
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    log::debug!("Request: {:?}", http_request);
    
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}