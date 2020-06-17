use {
    stry_backend::DataBackend,
    warp::{reply, Rejection, Reply},
};

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
