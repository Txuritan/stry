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
#[cfg_attr(feature = "types-postgres", derive(postgres_types::ToSql, postgres_types::FromSql))]
#[cfg_attr(feature = "types-postgres", postgres(name = "tag_type"))]
pub enum TagType {
    #[serde(rename = "warning")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "warning"))]
    Warning,
    #[serde(rename = "pairing")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "pairing"))]
    Pairing,
    #[serde(rename = "character")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "character"))]
    Character,
    #[serde(rename = "general")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "general"))]
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

#[cfg(feature = "types-sqlite")]
impl rusqlite::types::FromSql for TagType {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| match s.to_lowercase().as_str() {
                "warning" => Ok(TagType::Warning),
                "pairing" => Ok(TagType::Pairing),
                "character" => Ok(TagType::Character),
                "general" => Ok(TagType::General),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            })
    }
}

#[cfg(feature = "types-sqlite")]
impl rusqlite::types::ToSql for TagType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        match self {
            TagType::Warning => Ok("warning".into()),
            TagType::Pairing => Ok("pairing".into()),
            TagType::Character => Ok("character".into()),
            TagType::General => Ok("general".into()),
        }
    }
}