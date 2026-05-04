use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Member {
    pub(crate) id: String,
    pub(crate) server_id: String,
    pub(crate) user_id: String,
    pub(crate) position: Option<u32>,
    pub(crate) message_notifications: Option<u32>,
    pub(crate) suppress_everyone: Option<bool>,
    pub(crate) suppress_roles: Option<bool>,
    pub(crate) mobile_push: Option<bool>,
    pub(crate) is_banned: Option<bool>,
    pub(crate) banned_at: Option<String>,
    pub(crate) kicked_at: Option<String>,
    pub(crate) kick_expires_at: Option<String>,
    pub(crate) roles: Option<Vec<String>>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
}

impl Member {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn position(&self) -> Option<u32> {
        self.position
    }

    pub fn message_notifications(&self) -> Option<u32> {
        self.message_notifications
    }

    pub fn suppress_everyone(&self) -> Option<bool> {
        self.suppress_everyone
    }

    pub fn suppress_roles(&self) -> Option<bool> {
        self.suppress_roles
    }

    pub fn mobile_push(&self) -> Option<bool> {
        self.mobile_push
    }

    pub fn is_banned(&self) -> Option<bool> {
        self.is_banned
    }

    pub fn banned_at(&self) -> Option<&String> {
        self.banned_at.as_ref()
    }

    pub fn kicked_at(&self) -> Option<&String> {
        self.kicked_at.as_ref()
    }

    pub fn kick_expires_at(&self) -> Option<&String> {
        self.kick_expires_at.as_ref()
    }

    pub fn roles(&self) -> Option<&Vec<String>> {
        self.roles.as_ref()
    }

    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }
}
