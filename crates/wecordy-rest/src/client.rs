use super::{Error, Result};
use reqwest::{Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;
use url::Url;
use wecordy_api::models::ApiResponse;

#[derive(Debug)]
pub(crate) struct ClientRef {
    pub(crate) base_url: String,
    pub(crate) max_retries: u32,
    pub(crate) timeout: Duration,
    pub(crate) http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientRef>,
}

impl Client {
    pub async fn get<D>(&self, path: &str) -> Result<ApiResponse<D>>
    where
        D: DeserializeOwned,
    {
        Ok(self
            .make_request::<()>(Method::GET, path, None)
            .await?
            .json()
            .await?)
    }

    pub async fn post<B, D>(&self, path: &str, body: Option<&B>) -> Result<ApiResponse<D>>
    where
        B: Serialize,
        D: DeserializeOwned,
    {
        Ok(self
            .make_request(Method::POST, path, body)
            .await?
            .json()
            .await?)
    }

    pub async fn put<B, D>(&self, path: &str, body: Option<&B>) -> Result<ApiResponse<D>>
    where
        B: Serialize,
        D: DeserializeOwned,
    {
        Ok(self
            .make_request(Method::PUT, path, body)
            .await?
            .json()
            .await?)
    }

    pub async fn delete<B, D>(&self, path: &str, body: Option<&B>) -> Result<ApiResponse<D>>
    where
        B: Serialize,
        D: DeserializeOwned,
    {
        Ok(self
            .make_request(Method::DELETE, path, body)
            .await?
            .json()
            .await?)
    }

    pub async fn patch<B, D>(&self, path: &str, body: Option<&B>) -> Result<ApiResponse<D>>
    where
        B: Serialize,
        D: DeserializeOwned,
    {
        Ok(self
            .make_request(Method::PATCH, path, body)
            .await?
            .json()
            .await?)
    }

    async fn make_request<B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<Response>
    where
        B: serde::Serialize,
    {
        let mut attempt = 0;
        let full_url = format!("{}/{}", self.inner.base_url, path.trim_start_matches('/'));
        let url = Url::parse(&full_url)?;

        loop {
            let mut req = self.inner.http.request(method.clone(), url.as_str());
            if let Some(body) = body {
                req = req.json(body);
            }

            let response = req.send().await?;
            let status = response.status();

            if !status.is_success() {
                if status == StatusCode::TOO_MANY_REQUESTS {
                    let retry_after = self.parse_retry_after_header(&response)?;
                    debug!(
                        "Rate limit detected. A new request will be made in {} seconds.",
                        retry_after.as_secs()
                    );
                    tokio::time::sleep(retry_after).await;
                    continue;
                } else {
                    if attempt < self.inner.max_retries {
                        attempt += 1;
                        debug!("The request failed, a new request is being sent.");
                        continue;
                    } else {
                        debug!("The request failed, no more retries will be made.");
                        return Err(Error::UnexpectedResponse);
                    }
                }
            }

            return Ok(response);
        }
    }

    fn parse_retry_after_header(&self, response: &Response) -> Result<Duration> {
        let retry_after_value = response
            .headers()
            .get("retry-after")
            .ok_or_else(|| Error::HeaderMissing("retry-after".to_owned()))?;

        let retry_after_value = retry_after_value
            .to_str()
            .map_err(|_| Error::UnexpectedHeaderValue("retry-after".to_owned()))?;

        let retry_after_value = retry_after_value
            .parse::<u64>()
            .map_err(|_| Error::UnexpectedHeaderValue(retry_after_value.to_string()))?;

        Ok(Duration::from_secs(retry_after_value))
    }

    pub fn timeout(&self) -> Duration {
        self.inner.timeout
    }
}
