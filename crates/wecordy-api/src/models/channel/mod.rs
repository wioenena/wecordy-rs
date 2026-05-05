use serde::Deserialize;

mod channel_type;
mod permission_override;
pub use channel_type::ChannelType;
pub use permission_override::ChannelPermissionOverride;

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) topic: Option<String>,
    #[serde(rename = "type")]
    pub(crate) channel_type: ChannelType,
    pub(crate) server_id: String,
    pub(crate) parent_id: Option<String>,
    pub(crate) position: Option<u32>,
    pub(crate) user_limit: Option<u32>,
    pub(crate) slow_mode: Option<u32>,
    pub(crate) last_message_id: Option<String>,
    pub(crate) permission_overrides: Option<Vec<ChannelPermissionOverride>>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
}

impl Channel {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn topic(&self) -> Option<&String> {
        self.topic.as_ref()
    }

    pub fn channel_type(&self) -> ChannelType {
        self.channel_type
    }

    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    pub fn parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    pub fn position(&self) -> Option<u32> {
        self.position
    }

    pub fn user_limit(&self) -> Option<u32> {
        self.user_limit
    }

    pub fn slow_mode(&self) -> Option<u32> {
        self.slow_mode
    }

    pub fn last_message_id(&self) -> Option<&String> {
        self.last_message_id.as_ref()
    }

    pub fn permission_overrides(&self) -> Option<&Vec<ChannelPermissionOverride>> {
        self.permission_overrides.as_ref()
    }

    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }
}
