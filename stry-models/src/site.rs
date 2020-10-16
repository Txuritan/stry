#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Site {
    ArchiveOfOurOwn,
    FanFiction,
}

impl Site {
    pub fn as_str(self) -> &'static str {
        match self {
            Site::ArchiveOfOurOwn => "archive-of-our-own",
            Site::FanFiction => "fanfiction",
        }
    }
}
