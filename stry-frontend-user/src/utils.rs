use {
    std::future::Future,
    warp::{
        http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode},
        hyper::Body,
        Rejection, Reply,
    },
};

pub struct WebError {
    pub code: u32,
    pub icon: &'static str,
    pub help: &'static str,
}

#[inline(always)]
pub async fn wrap<Run, Fut, Res>(run: Run) -> Result<impl Reply, Rejection>
where
    Run: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<Res>>,
    Res: Reply,
{
    match run().await {
        Ok(res) => {
            let mut response = res.into_response();

            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));

            Ok(response)
        }
        Err(err) => {
            if let Err(err) = tokio::task::spawn_blocking(move || {
                let span = tracing::error_span!("Response error");
                let _enter = span.enter();

                for chain in err.chain() {
                    tracing::error!("{}", chain);
                }
            })
            .await
            {
                tracing::error!("Unable to join error thread: {}", err);
            }

            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            Ok(res)
        }
    }
}
