use chrono::{DateTime, Utc};

#[cfg(any(feature = "epub", feature = "json", feature = "messagepack"))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
    #[cfg(feature = "epub")]
    Epub,

    #[cfg(feature = "json")]
    Json,

    #[cfg(feature = "messagepack")]
    MessagePack,
}

#[cfg(any(feature = "epub", feature = "json", feature = "messagepack"))]
impl Format {
    pub fn variants() -> Vec<&'static str> {
        let mut var = Vec::with_capacity(3);

        #[cfg(feature = "epub")]
        {
            var.push("epub");
        }

        #[cfg(feature = "json")]
        {
            var.push("json");
        }

        #[cfg(feature = "messagepack")]
        {
            var.push("messagepack");
        }

        var
    }

    pub fn file_extension(self) -> &'static str {
        match self {
            #[cfg(feature = "epub")]
            Format::Epub => "epub",

            #[cfg(feature = "json")]
            Format::Json => "json",

            #[cfg(feature = "messagepack")]
            Format::MessagePack => "msgpk",
        }
    }
}

#[cfg(any(feature = "epub", feature = "json", feature = "messagepack"))]
impl std::str::FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "epub")]
            "epub" => Ok(Format::Epub),

            #[cfg(feature = "json")]
            "json" => Ok(Format::Json),

            #[cfg(feature = "messagepack")]
            "message-pack" => Ok(Format::MessagePack),

            _ => Err("Not a valid output format"),
        }
    }
}

#[cfg(any(feature = "epub", feature = "json", feature = "messagepack"))]
impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            #[cfg(feature = "epub")]
            Format::Epub => write!(f, "epub"),

            #[cfg(feature = "json")]
            Format::Json => write!(f, "json"),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => write!(f, "message-pack"),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Story {
    pub name: String,
    pub summary: String,

    pub language: Language,
    pub rating: Rating,
    pub state: State,

    pub chapters: Vec<Chapter>,
    pub words: u32,

    pub authors: Vec<String>,
    pub origins: Vec<String>,
    pub tags: Vec<(TagType, String)>,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Story {
    pub fn new(details: Details) -> Story {
        Story {
            name: details.name,
            summary: details.summary,

            language: details.language,
            rating: details.rating,
            state: details.state,

            chapters: Vec::with_capacity(details.chapters as usize),
            words: 0,

            authors: details.authors,
            origins: details.origins,
            tags: details.tags,

            series: None,

            created: details.created,
            updated: details.updated,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Details {
    pub name: String,
    pub summary: String,

    pub chapters: u32,

    pub language: Language,
    pub rating: Rating,
    pub state: State,

    pub authors: Vec<String>,
    pub origins: Vec<String>,
    pub tags: Vec<(TagType, String)>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Language {
    #[cfg_attr(feature = "serde", serde(rename = "english"))]
    English,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rating {
    #[cfg_attr(feature = "serde", serde(rename = "explicit"))]
    Explicit,
    #[cfg_attr(feature = "serde", serde(rename = "mature"))]
    Mature,
    #[cfg_attr(feature = "serde", serde(rename = "teen"))]
    Teen,
    #[cfg_attr(feature = "serde", serde(rename = "general"))]
    General,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum State {
    #[cfg_attr(feature = "serde", serde(rename = "completed"))]
    Completed,
    #[cfg_attr(feature = "serde", serde(rename = "in-progress"))]
    InProgress,
    #[cfg_attr(feature = "serde", serde(rename = "hiatus"))]
    Hiatus,
    #[cfg_attr(feature = "serde", serde(rename = "abandoned"))]
    Abandoned,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chapter {
    pub name: String,

    pub pre: String,
    pub main: String,
    pub post: String,

    pub words: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Series {
    pub name: String,

    pub summary: String,

    pub place: Option<i32>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TagType {
    #[cfg_attr(feature = "serde", serde(rename = "warning"))]
    Warning,
    #[cfg_attr(feature = "serde", serde(rename = "pairing"))]
    Pairing,
    #[cfg_attr(feature = "serde", serde(rename = "character"))]
    Character,
    #[cfg_attr(feature = "serde", serde(rename = "general"))]
    General,
}
