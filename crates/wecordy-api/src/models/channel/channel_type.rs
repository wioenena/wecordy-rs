use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ChannelType {
    Text,
    Voice,
    Category,
    Announcement,
    Stage,
    Forum,
    News,
    Rules,
} // TODO: implement methods for creating from and converting to a string.
