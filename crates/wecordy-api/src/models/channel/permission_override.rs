use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ChannelPermissionOverride {
    pub(crate) id: String,
    pub(crate) channel_id: String,
    pub(crate) role_id: Option<String>,
    pub(crate) user_id: Option<String>,
    pub(crate) permissions: HashMap<String, u32>,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl ChannelPermissionOverride {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn channel_id(&self) -> &String {
        &self.channel_id
    }

    pub fn role_id(&self) -> Option<&String> {
        self.role_id.as_ref()
    }

    pub fn user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }

    pub fn permissions(&self) -> &HashMap<String, u32> {
        &self.permissions
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }
}
