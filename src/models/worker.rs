use {
    crate::models::sync::Sites,
    chrono::{DateTime, Utc},
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Worker {
    pub id: String,

    pub task: Option<WorkerTask>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WorkerTask {
    pub id: String,

    pub name: String,
    pub site: Sites,
    pub url: String,

    pub chapter: i32,
    pub chapters: i32,
    pub next: Option<String>,

    pub completed: bool,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
