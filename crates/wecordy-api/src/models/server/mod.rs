pub mod application;
pub mod invite;

pub struct Server {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) is_public: Option<bool>,
    pub(crate) user_id: Option<String>,
    pub(crate) system_channel_id: Option<String>,
    pub(crate) avatar_url: Option<String>,
    pub(crate) banner_url: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
}

impl Server {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_public(&self) -> Option<bool> {
        self.is_public
    }

    pub fn user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }

    pub fn system_channel_id(&self) -> Option<&String> {
        self.system_channel_id.as_ref()
    }

    pub fn avatar_url(&self) -> Option<&String> {
        self.avatar_url.as_ref()
    }

    pub fn banner_url(&self) -> Option<&String> {
        self.banner_url.as_ref()
    }

    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }
}
