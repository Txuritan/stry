use {
    crate::{schema::Backend, Error, Readable},
    askama::Template,
    warp::{reject::custom, reply, Rejection, Reply},
};

pub async fn index(_backend: Backend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("index"))
}

pub async fn downloads(_backend: Backend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("downloads"))
}

pub async fn queue(_backend: Backend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("queue"))
}

pub async fn updates(_backend: Backend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("updates"))
}
