use {
    crate::{
        pages::dashboard,
        utils::{self, wrap},
    },
    chrono::Utc,
    stry_backend::DataBackend,
    warp::{reply, Rejection, Reply},
};

#[stry_macros::get("/about")]
pub async fn about(
    #[data] backend: DataBackend,
    #[header("Accept-Language")] languages: String,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let user_lang = utils::get_languages(&languages);

        let rendered: String =
            dashboard::About::new(time, &backend.details, user_lang).into_string()?;

        Ok(rendered)
    })
    .await
}

#[stry_macros::get("/")]
pub async fn index(
    #[data] _backend: DataBackend,
    #[header("Accept-Language")] _languages: String,
) -> Result<impl Reply, Rejection> {
    Ok(reply::html("index"))
}

#[stry_macros::get("/downloads")]
pub async fn downloads(
    #[data] _backend: DataBackend,
    #[header("Accept-Language")] _languages: String,
) -> Result<impl Reply, Rejection> {
    Ok(reply::html("downloads"))
}

#[stry_macros::get("/queue")]
pub async fn queue(
    #[data] _backend: DataBackend,
    #[header("Accept-Language")] _languages: String,
) -> Result<impl Reply, Rejection> {
    Ok(reply::html("queue"))
}

#[stry_macros::get("/updates")]
pub async fn updates(
    #[data] _backend: DataBackend,
    #[header("Accept-Language")] _languages: String,
) -> Result<impl Reply, Rejection> {
    Ok(reply::html("updates"))
}
