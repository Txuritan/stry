use {
    db_derive::Pool,
    warp::{reply, Rejection, Reply},
};

pub async fn index(_pool: Pool) -> Result<impl Reply, Rejection> {
    Ok(reply::html("index"))
}

pub async fn downloads(_pool: Pool) -> Result<impl Reply, Rejection> {
    Ok(reply::html("downloads"))
}

pub async fn queue(_pool: Pool) -> Result<impl Reply, Rejection> {
    Ok(reply::html("queue"))
}

pub async fn updates(_pool: Pool) -> Result<impl Reply, Rejection> {
    Ok(reply::html("updates"))
}
