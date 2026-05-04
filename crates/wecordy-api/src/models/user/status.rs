use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum UserStatus {
    Available,
    Busy,
    Afk,
    Offline,
} // TODO: implement methods for creating from and converting to a string.
