use {
    crate::models::Resource,
    chrono::{DateTime, Utc},
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: String,

    pub name: String,

    #[serde(rename = "type")]
    pub typ: TagType,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
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
        (self.typ.str(), self.typ.str())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label {}\" href=\"/tag/{}\">{}</a></li>",
            self.typ, self.id, self.name
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum TagType {
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "pairing")]
    Pairing,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "general")]
    General,
}

impl TagType {
    fn str(self) -> &'static str {
        match self {
            TagType::Warning => "background--red",
            TagType::Pairing => "background--yellow",
            TagType::Character => "background--purple",
            TagType::General => "background--gray",
        }
    }
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}
