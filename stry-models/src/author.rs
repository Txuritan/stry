use {
    crate::{generated::Author, List, Node},
    chrono::{TimeZone as _, Utc},
    std::fmt,
};

impl Author {
    pub fn new_test(id: impl Into<String>, name: impl Into<String>) -> Author {
        Author {
            id: id.into(),

            name: name.into(),

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        }
    }
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl Author {
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

// #[juniper::graphql_object(Context = DataBackend)]
// impl AuthorList {
//     pub fn total(&self) -> i32 {
//         self.total
//     }

//     pub fn items(&self) -> &[Author] {
//         &self.items
//     }
// }

impl From<List<Author>> for AuthorList {
    fn from(list: List<Author>) -> Self {
        AuthorList {
            total: list.total,
            items: list.items,
        }
    }
}
