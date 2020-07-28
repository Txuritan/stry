use {
    crate::{
        backend::DataBackend,
        models::{List, Resource},
    },
    chrono::{DateTime, Utc},
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Tag {
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

impl Resource for Tag {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    fn color(&self) -> (&str, &str) {
        ("background--gray", "background--gray")
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label background--gray\" href=\"/tag/{}\">{}</a></li>",
            self.id, self.name
        )
    }
}

pub struct TagList {
    pub total: i32,
    pub items: Vec<Tag>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl TagList {
    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn items(&self) -> &[Tag] {
        &self.items
    }
}

impl From<List<Tag>> for TagList {
    fn from(list: List<Tag>) -> Self {
        TagList {
            total: list.total,
            items: list.items,
        }
    }
}
