use super::ApplicationCommandOptionChoiceValue;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    pub(crate) name: String,
    pub(crate) value: ApplicationCommandOptionChoiceValue,
}

impl ApplicationCommandOptionChoice {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &ApplicationCommandOptionChoiceValue {
        &self.value
    }
}
