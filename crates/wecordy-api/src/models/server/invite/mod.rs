pub struct ServerInvite {
    pub(crate) id: String,
    pub(crate) server_id: String,
    pub(crate) url: String,
    pub(crate) expire: Option<String>,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl ServerInvite {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn server_id(&self) -> &String {
        &self.server_id
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn expire(&self) -> Option<&String> {
        self.expire.as_ref()
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }
}
