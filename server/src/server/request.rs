use {
    crate::typemap::{CloneAny, TypeMap},
    std::{collections::HashMap, sync::Arc},
};

pub struct Request<'a> {
    pub body: Option<String>,
    pub params: HashMap<&'a str, &'a str>,
    pub(crate) state: Arc<TypeMap<dyn CloneAny + Send + Sync>>,
    pub(crate) inner: &'a tiny_http::Request,
}

impl<'a> Request<'a> {
    pub fn state(&self) -> &TypeMap<dyn CloneAny + Send + Sync> {
        self.state.as_ref()
    }
}

impl<'a> std::ops::Deref for Request<'a> {
    type Target = tiny_http::Request;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
