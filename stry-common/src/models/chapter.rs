use chrono::{DateTime, Utc};

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: String,

    pub name: String,

    pub pre: String,
    pub main: String,
    pub post: String,

    pub words: i64,
    
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
