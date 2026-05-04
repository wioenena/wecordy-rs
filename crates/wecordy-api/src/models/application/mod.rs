use serde::Deserialize;

pub mod command;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) summary: String,
    pub(crate) avatar_url: String,
    pub(crate) owner_id: String,
    pub(crate) bot_user_id: String,
    pub(crate) client_id: String,
    pub(crate) is_public: bool,
    pub(crate) redirect_uris: Vec<String>,
    pub(crate) tos_url: String,
    pub(crate) privacy_policy_url: String,
    pub(crate) interactions_endpoint_url: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl Application {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn summary(&self) -> &String {
        &self.summary
    }

    pub fn avatar_url(&self) -> &String {
        &self.avatar_url
    }

    pub fn owner_id(&self) -> &String {
        &self.owner_id
    }

    pub fn bot_user_id(&self) -> &String {
        &self.bot_user_id
    }

    pub fn client_id(&self) -> &String {
        &self.client_id
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }

    pub fn redirect_uris(&self) -> &Vec<String> {
        &self.redirect_uris
    }

    pub fn tos_url(&self) -> &String {
        &self.tos_url
    }

    pub fn privacy_policy_url(&self) -> &String {
        &self.privacy_policy_url
    }

    pub fn interactions_endpoint_url(&self) -> &String {
        &self.interactions_endpoint_url
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }
}
