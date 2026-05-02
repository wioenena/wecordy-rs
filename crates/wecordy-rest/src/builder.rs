use std::sync::Arc;

pub struct ClientBuilder {
    token: Box<str>,
    timeout: Option<std::time::Duration>,
}

impl ClientBuilder {
    pub fn new(token: impl Into<Box<str>>) -> Self {
        Self {
            token: token.into(),
            timeout: None,
        }
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> crate::Result<crate::client::Client> {
        let timeout = self.timeout.unwrap_or(crate::constants::DEFAULT_TIMEOUT);

        let http = reqwest::ClientBuilder::new().timeout(timeout).build()?;

        let inner = crate::client::ClientRef {
            token: self.token,
            timeout,
            http,
        };

        Ok(crate::client::Client {
            inner: Arc::new(inner),
        })
    }
}
