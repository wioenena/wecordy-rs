use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub struct RouteMeta {
    pub(crate) method: http::Method,
    pub(crate) path: Cow<'static, str>,
}

impl RouteMeta {
    pub fn new(method: http::Method, path: Cow<'static, str>) -> Self {
        Self { method, path }
    }

    #[inline]
    pub fn method(&self) -> &http::Method {
        &self.method
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }
}
