use std::fmt;

use super::user::User;
use serde::{
    Deserialize,
    de::{Visitor, value::MapAccessDeserializer},
};

mod attachment;
mod reaction;
pub use attachment::MessageAttachment;
pub use reaction::MessageReaction;

#[derive(Debug)]
pub enum MessageUserId {
    Id(String),
    User(Box<User>),
}

struct MessageUserIdVisitor;

impl<'de> Visitor<'de> for MessageUserIdVisitor {
    type Value = MessageUserId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a user object")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessageUserId::Id(value.to_string()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let user = User::deserialize(MapAccessDeserializer::new(map))?;

        Ok(MessageUserId::User(Box::new(user)))
    }
}

impl<'de> Deserialize<'de> for MessageUserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(MessageUserIdVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub(crate) id: String,
    pub(crate) user_id: MessageUserId,
    pub(crate) webhook_id: Option<String>,
    pub(crate) to_user_id: Option<String>,
    pub(crate) channel_id: Option<String>,
    pub(crate) forum_post_id: Option<String>,
    pub(crate) text: String,
    pub(crate) reply_to_message_id: Option<String>,
    pub(crate) is_pinned: Option<bool>,
    pub(crate) attachments: Option<Vec<MessageAttachment>>,
    pub(crate) reactions: Option<Vec<MessageReaction>>,
    pub(crate) server_id: Option<String>,
    pub(crate) temp_id: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) edited_at: Option<String>,
}

impl Message {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn user_id(&self) -> &MessageUserId {
        &self.user_id
    }

    pub fn webhook_id(&self) -> Option<&String> {
        self.webhook_id.as_ref()
    }

    pub fn to_user_id(&self) -> Option<&String> {
        self.to_user_id.as_ref()
    }

    pub fn channel_id(&self) -> Option<&String> {
        self.channel_id.as_ref()
    }

    pub fn forum_post_id(&self) -> Option<&String> {
        self.forum_post_id.as_ref()
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn reply_to_message_id(&self) -> Option<&String> {
        self.reply_to_message_id.as_ref()
    }

    pub fn is_pinned(&self) -> Option<bool> {
        self.is_pinned
    }

    pub fn attachments(&self) -> Option<&Vec<MessageAttachment>> {
        self.attachments.as_ref()
    }

    pub fn reactions(&self) -> Option<&Vec<MessageReaction>> {
        self.reactions.as_ref()
    }

    pub fn server_id(&self) -> Option<&String> {
        self.server_id.as_ref()
    }

    pub fn temp_id(&self) -> Option<&String> {
        self.temp_id.as_ref()
    }

    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }

    pub fn edited_at(&self) -> Option<&String> {
        self.edited_at.as_ref()
    }
}
