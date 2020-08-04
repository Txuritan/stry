use {
    crate::{
        backend::DataBackend,
        models::{List, Node},
    },
    chrono::{DateTime, Utc},
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Author {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Author {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
}

impl Node for Author {
    fn id(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/authors/{}\">{}</a>", self.id, self.name)
    }
}

pub struct AuthorList {
    pub total: i32,
    pub items: Vec<Author>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl AuthorList {
    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn items(&self) -> &[Author] {
        &self.items
    }
}

impl From<List<Author>> for AuthorList {
    fn from(list: List<Author>) -> Self {
        AuthorList {
            total: list.total,
            items: list.items,
        }
    }
}
