use warp::{
    http::{
        header::{HeaderValue, CONTENT_TYPE, LOCATION},
        Response, StatusCode,
    },
    hyper::Body,
    Rejection, Reply,
};

pub enum Blocking {
    Text(String),
    Location(String),
}

impl Blocking {
    pub async fn spawn<F, O>(f: F) -> Result<impl Reply, Rejection>
    where
        F: FnOnce() -> anyhow::Result<O> + Send + 'static,
        O: Into<Blocking> + Send + 'static,
    {
        match tokio::task::spawn_blocking(f).await.unwrap() {
            Ok(ok) => match ok.into() {
                Blocking::Text(text) => {
                    let mut res = Response::new(Body::from(text));

                    res.headers_mut().insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("text/html; charset=utf-8"),
                    );

                    Ok(res)
                }
                Blocking::Location(loc) => {
                    let mut res = Response::new(Body::empty());

                    res.headers_mut()
                        .insert(LOCATION, HeaderValue::from_str(&loc).unwrap());

                    *res.status_mut() = StatusCode::MOVED_PERMANENTLY;

                    Ok(res)
                }
            },
            Err(err) => {
                tracing::error!("{}", err);

                let mut res = Response::new(Body::empty());

                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

                Ok(res)
            }
        }
    }
}

impl From<String> for Blocking {
    fn from(o: String) -> Blocking {
        Blocking::Text(o)
    }
}
