use std::fmt;
use std::time::Duration;

use reqwest::StatusCode;
use reqwest::blocking::Client as HttpClient;
use serde_json::{Value, json};

const DEFAULT_BASE_URL: &str = "https://api.rigby.host";

#[derive(Debug)]
pub struct SDKError {
    pub status: Option<u16>,
    pub body: String,
}

impl fmt::Display for SDKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Some(code) => write!(f, "rigby api error: status {}: {}", code, self.body),
            None => write!(f, "rigby api error: {}", self.body),
        }
    }
}

impl std::error::Error for SDKError {}

#[derive(Clone)]
pub struct Client {
    base_url: String,
    token: String,
    http: HttpClient,
}

impl Client {
    pub fn new(token: String, base_url: Option<String>) -> Self {
        let http = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build http client");

        Self {
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            token,
            http,
        }
    }

    pub fn call(&self, path: &[&str], data: Option<Value>) -> Result<Value, SDKError> {
        if path.is_empty() {
            return Err(SDKError {
                status: None,
                body: "path is required".to_string(),
            });
        }

        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), path.join("/"));
        let payload = json!({ "json": data.unwrap_or(Value::Null) });

        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&payload)
            .send()
            .map_err(|e| SDKError {
                status: None,
                body: e.to_string(),
            })?;

        let status = resp.status();
        let text = resp.text().unwrap_or_default();

        if !status.is_success() {
            return Err(SDKError {
                status: Some(status.as_u16()),
                body: text,
            });
        }

        if text.is_empty() {
            return Ok(json!(null));
        }

        if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
            if let Some(inner) = parsed.get("json") {
                return Ok(inner.clone());
            }
            return Ok(parsed);
        }

        Err(SDKError {
            status: Some(StatusCode::OK.as_u16()),
            body: text,
        })
    }
}
