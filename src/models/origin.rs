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
pub struct Origin {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Origin {
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

impl Resource for Origin {
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
        ("green-600", "green-600")
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/origin/{}\">{}</a>", self.id, self.name)
    }
}

pub struct OriginList {
    pub total: i32,
    pub items: Vec<Origin>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl OriginList {
    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn items(&self) -> &[Origin] {
        &self.items
    }
}

impl From<List<Origin>> for OriginList {
    fn from(list: List<Origin>) -> Self {
        OriginList {
            total: list.total,
            items: list.items,
        }
    }
}
