use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageAttachment {
    pub(crate) url: String,
    pub(crate) mimetype: String,
    pub(crate) size: u32,
}

impl MessageAttachment {
    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn mimetype(&self) -> &String {
        &self.mimetype
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
