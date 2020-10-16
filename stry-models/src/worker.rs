use {
    crate::{WorkerSite, WorkerTask},
    chrono::{DateTime, Utc},
    std::fmt,
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

impl WorkerSite {
    pub fn url(&self) -> &'static str {
        match self {
            WorkerSite::ArchiveOfOurOwn => "https://archiveofourown.org/",
            WorkerSite::FanFictionNet => "https://fanfiction.net/",
        }
    }
}

impl fmt::Display for WorkerSite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorkerSite::ArchiveOfOurOwn => write!(f, "Archive of Our Own"),
            WorkerSite::FanFictionNet => write!(f, "FanFiction.net"),
        }
    }
}
