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
pub struct Character {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Character {
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

impl Resource for Character {
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
        ("background--purple", "background--purple")
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label background--purple\" href=\"/character/{}\">{}</a></li>",
            self.id, self.name
        )
    }
}

pub struct CharacterList {
    pub total: i32,
    pub items: Vec<Character>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl CharacterList {
    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn items(&self) -> &[Character] {
        &self.items
    }
}

impl From<List<Character>> for CharacterList {
    fn from(list: List<Character>) -> Self {
        CharacterList {
            total: list.total,
            items: list.items,
        }
    }
}
