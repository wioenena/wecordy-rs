use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<D> {
    pub(crate) data: Option<D>,
    pub(crate) success: bool,
    pub(crate) message: String,
    pub(crate) status_code: u16,
    pub(crate) trace_id: String,
    pub(crate) errors: Vec<String>,
}

impl<D> ApiResponse<D>
where
    D: DeserializeOwned,
{
    pub fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn trace_id(&self) -> &String {
        &self.trace_id
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
}
