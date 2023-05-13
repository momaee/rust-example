use std::{
    env,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let port = match env::var("PORT") {
        Ok(var) => {var},
        Err(e) => {
            log::debug!("Failed to get PORT: {}", e);
            log::debug!("Using default port 8080");
            String::from("8080")
        }
    };

    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &port)?;

    log::info!("Listening on port {}", port);
    listen(listener)?;

    Ok(())
}

fn listen(listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?; 
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream)-> Result<(), Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| {
            match result {
                Ok(line) => line,
                Err(_) => String::from(""),
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();

    log::debug!("Request: {:?}", http_request);
    
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html")?;
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;

    stream.flush()?;

    Ok(())
}