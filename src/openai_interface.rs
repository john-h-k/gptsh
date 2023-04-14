use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::{json, Value};

pub struct OpenAiContext {
    pub api_key: String
}

pub async fn complete(context: &OpenAiContext, prompt: &str) -> Result<Value, reqwest::Error> {
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", context.api_key)).unwrap(),
    );

    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .headers(headers)
        .json(&json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "max_tokens": 1024,
            "temperature": 0
        }))
        .send()
        .await?;

    response.json::<Value>().await
}