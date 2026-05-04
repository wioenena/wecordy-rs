use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum InteractionOptionValue {
    String(String),
    Number(u32),
    Boolean(bool),
} // TODO: create custom serializer and deserializer
