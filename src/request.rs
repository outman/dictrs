use crate::yaml::{Config};
use crate::command::Args;
use serde::{Deserialize};
use reqwest::{header, Client, Error};
use serde_json::json;
use prettytable::{Table};

#[derive(Debug, Deserialize)]
struct TextCompletion {
    // id: String,
    // object: String,
    // created: i64,
    // model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
    // index: i32,
    // logprobs: Option<Value>,
    // finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    // prompt_tokens: i32,
    // completion_tokens: i32,
    total_tokens: i32,
}
pub async fn fetch(args: Args, config: Config) -> Result<(), Error> {
    let openai_api_key = config.openai_api_key;
    let openai_api_request_form = json!({
        "model": config.openai_api_request_form.model,
        "prompt": format!("Translate this From {} into {}:\n\n{}\n\n", args.source, args.target, args.content.join("")),
        "temperature": config.openai_api_request_form.temperature,
        "max_tokens": config.openai_api_request_form.max_tokens,
        "top_p": config.openai_api_request_form.top_p,
        "frequency_penalty": config.openai_api_request_form.frequency_penalty,
        "presence_penalty": config.openai_api_request_form.presence_penalty,
    });
    let client = Client::new();
    let response = client.post(config.openai_api_endpoint)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", openai_api_key))
        .body(openai_api_request_form.to_string())
        .send()
        .await?;

    if let Ok(body) = response.text().await {
        let tc: TextCompletion = serde_json::from_str(&body).unwrap();

        let mut table = Table::new();
        table.add_row(row!["Total tokens", tc.usage.total_tokens]);

        let mut result = Vec::new();
        for item in tc.choices {
            let v = item.text.clone();
            result.push(v.trim().to_string());
        }
        table.add_row(row!["Result", result.join("\n")]);
        table.printstd();
    }
    Ok(())
}