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
pub struct Warning {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Warning {
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

impl Resource for Warning {
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
        ("background--red", "background--red")
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label background--red\" href=\"/warning/{}\">{}</a></li>",
            self.id, self.name
        )
    }
}

pub struct WarningList {
    pub total: i32,
    pub items: Vec<Warning>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl WarningList {
    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn items(&self) -> &[Warning] {
        &self.items
    }
}

impl From<List<Warning>> for WarningList {
    fn from(list: List<Warning>) -> Self {
        WarningList {
            total: list.total,
            items: list.items,
        }
    }
}
