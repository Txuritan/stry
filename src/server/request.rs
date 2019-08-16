use {
    crate::typemap::{CloneAny, TypeMap},
    std::{collections::HashMap, sync::Arc},
};

pub struct Request<'a> {
    pub params: HashMap<&'a str, &'a str>,
    pub(crate) state: Arc<TypeMap<CloneAny + Send + Sync>>,
    pub(crate) inner: &'a crate::http::Request,
}

impl<'a> Request<'a> {
    pub fn state(&self) -> &TypeMap<CloneAny + Send + Sync> {
        self.state.as_ref()
    }
}

impl<'a> std::ops::Deref for Request<'a> {
    type Target = crate::http::Request;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
