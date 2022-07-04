use reqwest::{header::HeaderMap, Client};
use std::collections::HashMap;

///
pub async fn put(
    url: &str,
    data: &HashMap<&str, &str>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    let mut headers = HeaderMap::new();

    headers.insert(
        "Content-Type",
        "application/json;charset=UTF-8".parse().unwrap(),
    );

    client.put(url).headers(headers).json(data).send().await
}

pub async fn post(
    url: &str,
    data: &HashMap<&str, &str>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    let mut headers = HeaderMap::new();

    headers.insert(
        "Content-Type",
        "application/json;charset=UTF-8".parse().unwrap(),
    );

    client.post(url).headers(headers).json(data).send().await
}
