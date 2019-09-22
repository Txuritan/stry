pub mod middle;
pub mod request;
pub mod response;
pub mod router;
pub mod tree;

use {
    crate::{
        typemap::{CloneAny, TypeMap},
        Error,
    },
    std::{net::ToSocketAddrs, sync::Arc, thread},
};

pub use self::{request::Request, response::Response, router::Router};

pub struct Server {
    router: Arc<Router>,
    state: Arc<TypeMap<dyn CloneAny + Send + Sync>>,
    server: tiny_http::Server,
}

impl Server {
    pub fn new(
        addr: impl ToSocketAddrs + std::fmt::Display,
        state: TypeMap<dyn CloneAny + Send + Sync>,
        router: Router,
    ) -> Result<Self, Error> {
        log::info!("Binding to: {}", addr);

        Ok(Self {
            router: Arc::new(router),
            state: Arc::new(state),
            server: tiny_http::Server::http(addr)?,
        })
    }

    pub fn run(self) {
        log::info!("Starting server");

        let router = self.router.clone();
        let state = self.state.clone();

        for request in self.server.incoming_requests() {
            self.process(request, router.clone(), state.clone());
        }
    }

    #[allow(clippy::collapsible_if)]
    fn process(
        &self,
        mut req: tiny_http::Request,
        router: Arc<Router>,
        state: Arc<TypeMap<dyn CloneAny + Send + Sync>>,
    ) {
        thread::spawn(move || {
            if let Some(size) = req.body_length() {
                if size >= 1024 {
                    if let Err(err) = req.respond(
                        tiny_http::Response::from_string("400: Bad Request").with_status_code(400),
                    ) {
                        log::error!("{}", err);
                    }
                } else {
                    if req.data_reader.is_some() {
                        let mut buffer = String::new();

                        match req.as_reader().read_to_string(&mut buffer) {
                            Ok(_) => {
                                Self::handle(req, router, state, Some(buffer));
                            }
                            Err(reader_err) => {
                                if let Err(http_err) = req.respond(
                                    tiny_http::Response::from_string("500: Internal Server Error")
                                        .with_status_code(500),
                                ) {
                                    log::error!("http: {}, reader: {}", http_err, reader_err);
                                }
                            }
                        }
                    }
                }
            } else {
                Self::handle(req, router, state, None);
            }
        });
    }

    #[allow(clippy::collapsible_if)]
    fn handle(
        req: tiny_http::Request,
        router: Arc<Router>,
        state: Arc<TypeMap<dyn CloneAny + Send + Sync>>,
        body: Option<String>,
    ) {
        let me = req.method();
        let url = req.url();

        if let Some((handler, params)) = router.find(me, url) {
            let new_req = Request {
                body,
                params,
                state,
                inner: &req,
            };

            let res = handler.as_ref().handle(new_req);

            if let Err(err) = match res {
                Ok(ok) => {
                    let inner = ok.into_inner();

                    log::info!("{} {} {}", me, inner.status_code().0, url);

                    req.respond(inner)
                }
                Err(err) => {
                    log::info!("{} {} {}", me, 500, url);

                    log::error!("{}", err);

                    req.respond(
                        tiny_http::Response::from_string("500: Internal Server Error")
                            .with_status_code(500),
                    )
                }
            } {
                log::error!("{}", err);
            }
        } else {
            log::info!("{} {} {}", me, 404, url);

            if let Err(err) = req.respond(
                tiny_http::Response::from_string("404: Page Not Found").with_status_code(404),
            ) {
                log::error!("{}", err);
            }
        }
    }
}
