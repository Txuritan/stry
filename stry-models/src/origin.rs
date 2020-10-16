use {
    crate::{List, Origin},
    chrono::{TimeZone as _, Utc},
    std::fmt,
};

impl Origin {
    pub fn new_test(id: impl Into<String>, name: impl Into<String>) -> Origin {
        Origin {
            id: id.into(),

            name: name.into(),

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        }
    }
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl Origin {
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

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/origins/{}\">{}</a>", self.id, self.name)
    }
}

pub struct OriginList {
    pub total: i32,
    pub items: Vec<Origin>,
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl OriginList {
//     pub fn total(&self) -> i32 {
//         self.total
//     }

//     pub fn items(&self) -> &[Origin] {
//         &self.items
//     }
// }

impl From<List<Origin>> for OriginList {
    fn from(list: List<Origin>) -> Self {
        OriginList {
            total: list.total,
            items: list.items,
        }
    }
}
