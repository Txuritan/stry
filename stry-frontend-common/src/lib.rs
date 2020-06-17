use {
    std::future::Future,
    warp::{
        http::{Response, StatusCode},
        hyper::Body,
        Rejection, Reply,
    },
};

pub async fn wrap<Run, Fut, Res>(run: Run) -> Result<impl Reply, Rejection>
where
    Run: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<Res>>,
    Res: Reply,
{
    match run().await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => {
            // TODO: some how make this better, don't understand tracing well enough
            tracing::error!("{}", err);

            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            Ok(res)
        }
    }
}
