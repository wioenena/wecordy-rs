use serde::Deserialize;

mod change;
pub use change::AuditLogChange;

#[derive(Debug, Deserialize)]
pub struct AuditLog {
    pub(crate) id: String,
    pub(crate) server_id: String,
    pub(crate) user_id: String,
    pub(crate) action_type: String,
    pub(crate) channel_id: Option<String>,
    pub(crate) role_id: Option<String>,
    pub(crate) target_user_id: Option<String>,
    pub(crate) changes: Option<Vec<AuditLogChange>>,
    pub(crate) reason: Option<String>,
    pub(crate) created_at: String,
}

impl AuditLog {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    pub fn user_id(&self) -> &String {
        &self.user_id
    }

    pub fn action_type(&self) -> &String {
        &self.action_type
    }

    pub fn channel_id(&self) -> Option<&String> {
        self.channel_id.as_ref()
    }

    pub fn role_id(&self) -> Option<&String> {
        self.role_id.as_ref()
    }

    pub fn target_user_id(&self) -> Option<&String> {
        self.target_user_id.as_ref()
    }

    pub fn changes(&self) -> Option<&Vec<AuditLogChange>> {
        self.changes.as_ref()
    }

    pub fn reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }
}
