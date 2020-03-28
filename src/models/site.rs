use {
    parking_lot::Mutex,
    std::{fmt, sync::Arc},
    story_dl::Uri,
};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
#[derive(enum_map::Enum)]
pub enum Site {
    #[kind(rename = "archive-of-our-own")]
    ArchiveOfOurOwn,

    #[kind(rename = "fanfiction")]
    FanFiction,
}

impl Site {
    pub fn as_str(self) -> &'static str {
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

    pub fn as_mutex(self) -> MutexSite {
        self.into()
    }
}

#[derive(Clone)]
pub struct MutexSite {
    inner: Arc<Mutex<Option<Site>>>,
}

impl MutexSite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self) -> Option<Site> {
        *self.inner.lock()
    }

    pub fn store(&self, site: Option<Site>) {
        println!("store");

        let mut inner = self.inner.lock();
        *inner = site;
    }

    pub fn empty(&self) {
        let mut inner = self.inner.lock();
        *inner = None;
    }
}

impl From<Site> for MutexSite {
    fn from(site: Site) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Some(site))),
        }
    }
}

impl Default for MutexSite {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }
}

impl fmt::Debug for MutexSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.load())
    }
}

impl PartialEq<Option<Site>> for MutexSite {
    fn eq(&self, site: &Option<Site>) -> bool {
        self.load() == *site
    }
}

impl PartialEq<Site> for MutexSite {
    fn eq(&self, site: &Site) -> bool {
        self.load() == Some(*site)
    }
}
