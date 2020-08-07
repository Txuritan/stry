use {
    crate::{
        backend::DataBackend,
        frontend::user::{pages::dashboard, utils::wrap},
    },
    askama::Template,
    chrono::Utc,
    warp::{reply, Rejection, Reply},
};

pub async fn about(pool: DataBackend) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let rendered: String = dashboard::About::new(time, &pool.versions).render()?;

        Ok(rendered)
    })
    .await
}

pub async fn index(_pool: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("index"))
}

pub async fn downloads(_pool: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("downloads"))
}

pub async fn queue(_pool: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("queue"))
}

pub async fn updates(_pool: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("updates"))
}
