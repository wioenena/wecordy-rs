use serde::Deserialize;

mod data;
mod option;
mod option_value;

pub use data::InteractionData;
pub use option::InteractionOption;
pub use option_value::InteractionOptionValue;

#[derive(Debug, Deserialize)]
pub struct Interaction {
    pub(crate) id: String,
    pub(crate) interaction_type: u32,
    pub(crate) token: String,
    pub(crate) application_id: String,
    pub(crate) channel_id: Option<String>,
    pub(crate) server_id: Option<String>,
    pub(crate) user: Option<super::user::User>,
    pub(crate) member: Option<super::member::Member>,
    pub(crate) data: Option<InteractionData>,
}

impl Interaction {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn interaction_type(&self) -> u32 {
        self.interaction_type
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn application_id(&self) -> &String {
        &self.application_id
    }

    pub fn channel_id(&self) -> Option<&String> {
        self.channel_id.as_ref()
    }

    pub fn server_id(&self) -> Option<&String> {
        self.server_id.as_ref()
    }

    pub fn user(&self) -> Option<&super::user::User> {
        self.user.as_ref()
    }

    pub fn member(&self) -> Option<&super::member::Member> {
        self.member.as_ref()
    }

    pub fn data(&self) -> Option<&InteractionData> {
        self.data.as_ref()
    }
}
