use {
    crate::{List, Warning},
    chrono::{TimeZone as _, Utc},
    std::fmt,
};

impl Warning {
    pub fn new_test(id: impl Into<String>, name: impl Into<String>) -> Warning {
        Warning {
            id: id.into(),

            name: name.into(),

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        }
    }
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl Warning {
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

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label color__red\" href=\"/warnings/{}\">{}</a></li>",
            self.id, self.name
        )
    }
}

pub struct WarningList {
    pub total: i32,
    pub items: Vec<Warning>,
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl WarningList {
//     pub fn total(&self) -> i32 {
//         self.total
//     }

//     pub fn items(&self) -> &[Warning] {
//         &self.items
//     }
// }

impl From<List<Warning>> for WarningList {
    fn from(list: List<Warning>) -> Self {
        WarningList {
            total: list.total,
            items: list.items,
        }
    }
}
