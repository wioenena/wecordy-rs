use super::ApplicationCommandOptionChoice;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplicationCommandOption {
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "type")]
    pub(crate) option_type: u32,
    pub(crate) required: Option<bool>,
    pub(crate) choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub(crate) options: Option<Vec<ApplicationCommandOption>>,
    pub(crate) min_value: Option<u32>,
    pub(crate) max_value: Option<u32>,
    pub(crate) min_length: Option<u32>,
    pub(crate) max_length: Option<u32>,
}

impl ApplicationCommandOption {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn option_type(&self) -> u32 {
        self.option_type
    }

    pub fn required(&self) -> Option<bool> {
        self.required
    }

    pub fn choices(&self) -> Option<&Vec<ApplicationCommandOptionChoice>> {
        self.choices.as_ref()
    }

    pub fn options(&self) -> Option<&Vec<ApplicationCommandOption>> {
        self.options.as_ref()
    }

    pub fn min_value(&self) -> Option<u32> {
        self.min_value
    }

    pub fn max_value(&self) -> Option<u32> {
        self.max_value
    }

    pub fn min_length(&self) -> Option<u32> {
        self.min_length
    }

    pub fn max_length(&self) -> Option<u32> {
        self.max_length
    }
}
