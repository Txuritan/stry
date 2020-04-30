use chrono::{DateTime, Utc};

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Notification {
    pub id: String,
    
    #[serde(rename = "level")]
    pub level: Level,

    pub head: String,
    pub body: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Level {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "general")]
    General,
}
