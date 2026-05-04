use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Number(u32),
} // TODO: create custom serialize and deserialize functions
