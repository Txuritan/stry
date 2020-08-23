use {
    crate::{
        backend::DataBackend,
        frontend::user::{pages::dashboard, utils::wrap},
    },
    askama::Template,
    chrono::Utc,
    warp::{reply, Rejection, Reply},
};

#[warp_macros::get("/about")]
pub async fn about(#[data] backend: DataBackend) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let rendered: String = dashboard::About::new(time, &backend.versions).render()?;

        Ok(rendered)
    })
    .await
}

#[warp_macros::get("/")]
pub async fn index(#[data] _backend: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("index"))
}

#[warp_macros::get("/downloads")]
pub async fn downloads(#[data] _backend: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("downloads"))
}

#[warp_macros::get("/queue")]
pub async fn queue(#[data] _backend: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("queue"))
}

#[warp_macros::get("/updates")]
pub async fn updates(#[data] _backend: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("updates"))
}
