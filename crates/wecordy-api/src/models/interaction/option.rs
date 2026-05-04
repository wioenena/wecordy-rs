use super::InteractionOptionValue;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InteractionOption {
    pub(crate) name: String,
    pub(crate) option_type: u32,
    pub(crate) value: Option<InteractionOptionValue>,
    pub(crate) options: Option<Vec<InteractionOption>>,
}

impl InteractionOption {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn option_type(&self) -> u32 {
        self.option_type
    }

    pub fn value(&self) -> Option<&InteractionOptionValue> {
        self.value.as_ref()
    }

    pub fn options(&self) -> Option<&Vec<InteractionOption>> {
        self.options.as_ref()
    }
}
