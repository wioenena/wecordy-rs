use super::InteractionOption;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InteractionData {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) data_type: Option<u32>,
    pub(crate) options: Option<Vec<InteractionOption>>,
}

impl InteractionData {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn data_type(&self) -> Option<u32> {
        self.data_type
    }

    pub fn options(&self) -> Option<&Vec<InteractionOption>> {
        self.options.as_ref()
    }
}
