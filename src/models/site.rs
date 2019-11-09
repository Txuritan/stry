use {
    http_req::uri::Uri,
    rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};

pub type SiteMap<V> = enum_map::EnumMap<Site, V>;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(enum_map::Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "site")]
pub enum Site {
    #[postgres(name = "archive-of-our-own")]
    ArchiveOfOurOwn,

    #[postgres(name = "fanfiction")]
    FanFiction,
}

impl Site {
    pub fn as_str(&self) -> &'static str {
        match self {
            Site::ArchiveOfOurOwn => "archive-of-our-own",
            Site::FanFiction => "fanfiction",
        }
    }

    pub fn from_url(url: &str) -> Option<Self> {
        url.parse::<Uri>()
            .ok()
            .and_then(|uri| uri.host().map(String::from))
            .and_then(|host| match host.as_str() {
                "archiveofourown.org" | "www.archiveofourown.org" => Some(Site::ArchiveOfOurOwn),
                "fanfiction.net" | "www.fanfiction.net" | "m.fanfiction.net" => {
                    Some(Site::FanFiction)
                }
                _ => None,
            })
    }
}

impl FromSql for Site {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "archive-of-our-own" => Site::ArchiveOfOurOwn,
            "fanfiction" => Site::FanFiction,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Site {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(self.as_str().into())
    }
}
