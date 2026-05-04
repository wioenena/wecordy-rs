use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Role {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) color: Option<String>,
    pub(crate) position: Option<u32>,
    pub(crate) permissions: Option<HashMap<String, u32>>,
    pub(crate) server_id: String,
    pub(crate) is_default: Option<bool>,
    pub(crate) bot_id: Option<String>,
    pub(crate) is_managed: Option<bool>,
    pub(crate) created_at: Option<String>,
    pub(crate) updated_at: Option<String>,
}

impl Role {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn position(&self) -> Option<u32> {
        self.position
    }

    pub fn permissions(&self) -> Option<&HashMap<String, u32>> {
        self.permissions.as_ref()
    }

    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    pub fn is_default(&self) -> Option<bool> {
        self.is_default
    }

    pub fn bot_id(&self) -> Option<&String> {
        self.bot_id.as_ref()
    }

    pub fn is_managed(&self) -> Option<bool> {
        self.is_managed
    }

    pub fn created_at(&self) -> Option<&String> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&String> {
        self.updated_at.as_ref()
    }
}
