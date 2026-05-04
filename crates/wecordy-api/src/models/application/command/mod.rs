use serde::Deserialize;

mod option;
mod option_choice;
mod option_choice_value;

pub use option::ApplicationCommandOption;
pub use option_choice::ApplicationCommandOptionChoice;
pub use option_choice_value::ApplicationCommandOptionChoiceValue;

#[derive(Debug, Deserialize)]
pub struct ApplicationCommand {
    pub(crate) id: String,
    pub(crate) application_id: String,
    pub(crate) server_id: Option<String>,
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "type")]
    pub(crate) command_type: u32,
    pub(crate) options: Vec<ApplicationCommandOption>,
    pub(crate) default_member_permissions: Option<String>,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl ApplicationCommand {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn application_id(&self) -> &String {
        &self.application_id
    }

    pub fn server_id(&self) -> Option<&String> {
        self.server_id.as_ref()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn command_type(&self) -> u32 {
        self.command_type
    }

    pub fn options(&self) -> &Vec<ApplicationCommandOption> {
        &self.options
    }

    pub fn default_member_permissions(&self) -> Option<&String> {
        self.default_member_permissions.as_ref()
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }
}
