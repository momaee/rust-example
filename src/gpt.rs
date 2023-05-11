use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    model: String,
    messages: Vec<Message>,
    temperature: i32,
    top_p: i32,
    n: i32,
    stream: bool,
    max_tokens: i32,
    presence_penalty: i32,
    frequency_penalty: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    id: String,
    object: String,
    created: i32,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: i32,
}

pub async fn chat(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    let gpt_api_key = env::var("OPENAI_API_KEY")?;
    
    // todo: check to see if can create a client once and reuse it
    let client = Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Accept", "application/json".parse()?);
    headers.insert("Authorization", ("Bearer ".to_string() + &gpt_api_key).parse()?);

    let request = Request {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: msg.to_string(),
            }
        ],
        temperature: 1,
        top_p: 1,
        n: 1,
        stream: false,
        max_tokens: 250,
        presence_penalty: 0,
        frequency_penalty: 0,
    };


    let body = client.request(reqwest::Method::POST, "https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&request)
        .send().await?
        .text().await?;

    println!("{}", body);

    let response: Response = serde_json::from_str(&body)?;

    let mut body = String::new();

    for choice in response.choices {
        body.push_str(&choice.message.content);
    }

    Ok(body)
}