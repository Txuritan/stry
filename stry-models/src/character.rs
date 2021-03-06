use {
    crate::{Character, List},
    chrono::{TimeZone as _, Utc},
    std::fmt,
};

impl Character {
    pub fn new_test(id: impl Into<String>, name: impl Into<String>) -> Character {
        Character {
            id: id.into(),

            name: name.into(),

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        }
    }
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl Character {
//     pub fn id(&self) -> &str {
//         &self.id
//     }

//     pub fn name(&self) -> &str {
//         &self.name
//     }

//     pub fn created(&self) -> &DateTime<Utc> {
//         &self.created
//     }

//     pub fn updated(&self) -> &DateTime<Utc> {
//         &self.updated
//     }
// }

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label color__purple\" href=\"/characters/{}\">{}</a></li>",
            self.id, self.name,
        )
    }
}

pub struct CharacterList {
    pub total: i32,
    pub items: Vec<Character>,
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl CharacterList {
//     pub fn total(&self) -> i32 {
//         self.total
//     }

//     pub fn items(&self) -> &[Character] {
//         &self.items
//     }
// }

impl From<List<Character>> for CharacterList {
    fn from(list: List<Character>) -> Self {
        CharacterList {
            total: list.total,
            items: list.items,
        }
    }
}
