use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct ClientRef {
    pub(crate) token: Box<str>,
    pub(crate) timeout: Duration,
    pub(crate) http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientRef>,
}

impl Client {
    #[inline]
    pub fn token(&self) -> &str {
        &self.inner.token
    }

    #[inline]
    pub fn timeout(&self) -> Duration {
        self.inner.timeout
    }
}
