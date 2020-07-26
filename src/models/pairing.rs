use {
    crate::{backend::DataBackend, models::Character},
    chrono::{DateTime, Utc},
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Pairing {
    pub id: String,

    pub characters: Vec<Character>,

    pub platonic: bool,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Pairing {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn characters(&self) -> &[Character] {
        &self.characters
    }

    pub fn platonic(&self) -> bool {
        self.platonic
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
}

impl fmt::Display for Pairing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label background--yellow\" href=\"/pairing/{}\">{}</a></li>",
            self.id,
            self.characters
                .iter()
                .map(|c| &*c.name)
                .collect::<Vec<&str>>()
                .join("/")
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PairingPart {
    pub id: String,

    pub platonic: bool,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
