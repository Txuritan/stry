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
    state: Arc<TypeMap<CloneAny + Send + Sync>>,
    server: crate::http::Server,
}

impl Server {
    pub fn new(
        addr: impl ToSocketAddrs + std::fmt::Display,
        state: TypeMap<CloneAny + Send + Sync>,
        router: Router,
    ) -> Result<Self, Error> {
        log::info!("Binding to: {}", addr);

        Ok(Self {
            router: Arc::new(router),
            state: Arc::new(state),
            server: crate::http::Server::http(addr)?,
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
        req: crate::http::Request,
        router: Arc<Router>,
        state: Arc<TypeMap<CloneAny + Send + Sync>>,
    ) {
        thread::spawn(move || {
            let me = req.method();
            let url = req.url();

            if let Some(route) = router.find(me, url) {
                let new_req = Request {
                    params: route.1,
                    state,
                    inner: &req,
                };

                let res = route.0.as_ref().handle(new_req);

                if let Err(err) = match res {
                    Ok(ok) => req.respond(ok.into_inner()),
                    Err(err) => {
                        log::error!("{}", err);

                        req.respond(
                            crate::http::Response::from_string("500: Internal Server Error")
                                .with_status_code(500),
                        )
                    }
                } {
                    log::error!("{}", err);
                }
            } else {
                if let Err(err) = req.respond(
                    crate::http::Response::from_string("404: Page Not Found").with_status_code(404),
                ) {
                    log::error!("{}", err);
                }
            }
        });
    }
}
