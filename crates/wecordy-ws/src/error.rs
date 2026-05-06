#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already connected")]
    AlreadyConnected,
    #[error("not connected")]
    NotConnected,
    #[error("socket error: {0}")]
    Socket(#[from] rust_socketio::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
