#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest request error: {0}")]
    ReqwestRequest(#[from] reqwest::Error),
    #[error("reqwest header name error: {0}")]
    ReqwestHeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error("reqwest header value error: {0}")]
    ReqwestHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("cannot parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("unexpected header value: {0}")]
    UnexpectedHeaderValue(String),
    #[error("header missing: {0}")]
    HeaderMissing(String),
    #[error("reqwest response content unexpected")]
    UnexpectedResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
