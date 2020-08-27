use chrono::{DateTime, Utc};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: String,

    pub name: String,

    pub pre: String,
    pub main: String,
    pub post: String,

    pub words: i32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl Chapter {
//     pub fn id(&self) -> &str {
//         &self.id
//     }

//     pub fn name(&self) -> &str {
//         &self.name
//     }

//     pub fn pre(&self) -> &str {
//         &self.pre
//     }

//     pub fn main(&self) -> &str {
//         &self.main
//     }

//     pub fn post(&self) -> &str {
//         &self.post
//     }

//     pub fn words(&self) -> i32 {
//         self.words
//     }

//     pub fn created(&self) -> &DateTime<Utc> {
//         &self.created
//     }

//     pub fn updated(&self) -> &DateTime<Utc> {
//         &self.updated
//     }
// }
