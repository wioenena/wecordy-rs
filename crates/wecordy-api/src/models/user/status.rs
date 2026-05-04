use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum UserStatus {
    Available,
    Busy,
    Afk,
    Offline,
}
