use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerBot {
    pub(crate) id: String,
    pub(crate) application_id: String,
    pub(crate) server_id: String,
    pub(crate) bot_user_id: String,
    pub(crate) permissions: u32,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl ServerBot {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn application_id(&self) -> &String {
        &self.application_id
    }

    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    pub fn bot_user_id(&self) -> &String {
        &self.bot_user_id
    }

    pub fn permissions(&self) -> u32 {
        self.permissions
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }
}
