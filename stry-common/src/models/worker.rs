use {
    chrono::{DateTime, Utc},
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WorkerTask {
    pub id: String,

    pub site: String,
    pub url: String,
    pub chapter: u32,

    pub complete: bool,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
