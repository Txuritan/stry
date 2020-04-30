use chrono::{DateTime, Utc};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Series {
    pub id: String,

    pub name: String,

    pub summary: String,

    pub place: i32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
