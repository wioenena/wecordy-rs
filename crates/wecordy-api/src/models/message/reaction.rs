use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageReaction {
    pub(crate) id: String,
    pub(crate) message_id: String,
    pub(crate) user_id: String,
    pub(crate) emoji: String,
    pub(crate) created_at: String,
}

impl MessageReaction {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn message_id(&self) -> &String {
        &self.message_id
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }
}
