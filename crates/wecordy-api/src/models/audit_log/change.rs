use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuditLogChange {
    pub(crate) key: String,
    pub(crate) old_value: Option<String>, // TODO: what is type?
    pub(crate) new_value: Option<String>, // TODO: what is type?
}

impl AuditLogChange {
    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn old_value(&self) -> Option<&String> {
        self.old_value.as_ref()
    }

    pub fn new_value(&self) -> Option<&String> {
        self.new_value.as_ref()
    }
}
