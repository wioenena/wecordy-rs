use serde::Deserialize;

mod status;
pub use status::UserStatus;

#[derive(Debug, Deserialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) full_name: Option<String>,
    pub(crate) username: String,
    pub(crate) email: Option<String>,
    pub(crate) status: Option<UserStatus>,
    pub(crate) is_active: Option<bool>,
    pub(crate) is_bot: Option<bool>,
    pub(crate) is_nitro: Option<bool>,
    pub(crate) avatar_url: Option<String>,
    pub(crate) banner_url: Option<String>,
    pub(crate) bio: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
}

impl User {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn first_name(&self) -> Option<&String> {
        self.first_name.as_ref()
    }

    pub fn last_name(&self) -> Option<&String> {
        self.last_name.as_ref()
    }

    pub fn full_name(&self) -> Option<&String> {
        self.full_name.as_ref()
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    pub fn status(&self) -> Option<UserStatus> {
        self.status
    }

    pub fn is_active(&self) -> Option<bool> {
        self.is_active
    }

    pub fn is_bot(&self) -> Option<bool> {
        self.is_bot
    }

    pub fn is_nitro(&self) -> Option<bool> {
        self.is_nitro
    }

    pub fn avatar_url(&self) -> Option<&String> {
        self.avatar_url.as_ref()
    }

    pub fn banner_url(&self) -> Option<&String> {
        self.banner_url.as_ref()
    }

    pub fn bio(&self) -> Option<&String> {
        self.bio.as_ref()
    }

    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }
}
