use reqwest::header::{self, HeaderValue};
use std::sync::Arc;

pub struct ClientBuilder {
    token: String,
    base_url: Option<String>,
    max_retries: Option<u32>,
    timeout: Option<std::time::Duration>,
}

impl ClientBuilder {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: None,
            max_retries: None,
            timeout: None,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> crate::Result<crate::client::Client> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bot {}", &self.token))?,
        );

        let timeout = self.timeout.unwrap_or(crate::constants::DEFAULT_TIMEOUT);

        let http = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(timeout)
            .build()?;

        let max_retries = self.max_retries.unwrap_or(3);
        let base_url = self
            .base_url
            .unwrap_or_else(|| wecordy_api::constants::API_URL.to_owned())
            .trim_end_matches('/')
            .to_owned();

        let inner = crate::client::ClientRef {
            max_retries,
            base_url,
            timeout,
            http,
        };

        Ok(crate::client::Client {
            inner: Arc::new(inner),
        })
    }
}
