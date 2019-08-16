use {
    super::{tree::PathTree, Request, Response},
    crate::{Error, http::Method},
    std::{collections::HashMap, sync::Arc},
};

type RouteHandler = Fn(Request) -> Result<Response, Error> + Send + Sync + 'static;

pub struct Route {
    inner: Box<RouteHandler>,
}

impl Route {
    pub fn handle(&self, req: Request) -> Result<Response, Error> {
        ((*self).inner)(req)
    }
}

#[derive(Clone)]
pub struct Router {
    path: String,
    trees: HashMap<Method, PathTree<Arc<Route>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
            path: "/".to_owned(),
        }
    }

    // scope with prefix
    pub fn scope(mut self, path: &str, builder: impl FnOnce(&mut Router)) -> Self {
        let mut group = Router {
            trees: self.trees.clone(),
            path: join_paths(&self.path, path),
        };

        builder(&mut group);

        self.trees = group.trees;

        self
    }

    fn _handle(
        mut self,
        method: Method,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        // TODO: combine middleware + handler to finally handler
        self.trees
            .entry(method)
            .or_insert_with(PathTree::new)
            .insert(
                path,
                Arc::new(Route {
                    inner: Box::new(handler),
                }),
            );

        self
    }

    pub fn handle(
        self,
        method: Method,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        let path = join_paths(&self.path, path);
        self._handle(method, &path, handler)
    }

    pub fn get(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Get, path, handler)
    }

    pub fn post(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Post, path, handler)
    }

    pub fn delete(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Delete, path, handler)
    }

    pub fn patch(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Patch, path, handler)
    }

    pub fn put(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Put, path, handler)
    }

    pub fn options(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Options, path, handler)
    }

    pub fn head(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Head, path, handler)
    }

    pub fn connect(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Connect, path, handler)
    }

    pub fn trace(
        self,
        path: &str,
        handler: impl Fn(Request) -> Result<Response, Error> + Send + Sync + 'static,
    ) -> Self {
        self.handle(Method::Trace, path, handler)
    }

    pub fn find<'a>(
        &'a self,
        method: &'a crate::http::Method,
        path: &'a str,
    ) -> Option<(Arc<Route>, HashMap<&'a str, &'a str>)> {
        let tree = self.trees.get(&method)?;

        if let Some((r, v)) = tree.find(path) {
            Some((r.clone(), v.into_iter().collect()))
        } else {
            None
        }
    }
}

fn join_paths(a: &str, mut b: &str) -> String {
    if b.is_empty() {
        return a.to_owned();
    }
    b = b.trim_start_matches('/');
    a.trim_end_matches('/').to_owned() + "/" + b
}
