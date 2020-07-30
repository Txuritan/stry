pub mod archive_of_our_own;
pub mod fanfiction;

use {
    crate::workers::scraper::{
        models::{Chapter, Details},
        utils::req,
        Uri,
    },
    std::{convert::TryInto, fmt, sync::Arc},
};

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Sites {
    ArchiveOfOurOwn,
    FanFictionNet,
}

impl Sites {
    pub fn url(&self) -> &'static str {
        match self {
            Sites::ArchiveOfOurOwn => "https://archiveofourown.org/",
            Sites::FanFictionNet => "https://fanfiction.net/",
        }
    }
}

impl fmt::Display for Sites {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sites::ArchiveOfOurOwn => write!(f, "Archive of Our Own"),
            Sites::FanFictionNet => write!(f, "FanFiction.net"),
        }
    }
}

pub struct SiteRef {
    inner: Inner,
}

impl SiteRef {
    pub async fn get_details(&mut self) -> anyhow::Result<Details> {
        match &mut self.inner {
            Inner::ArchiveOfOurOwn { id, document } => {
                if document.is_none() {
                    let url = format!(
                        "https://archiveofourown.org/works/{}?view_full_work=true",
                        id
                    )
                    .parse::<Uri>()?;

                    let body = req(&url).await?;

                    *document = Some(Arc::new(body));
                }

                let document = document.clone().expect("This should not be `None`");

                let details =
                    tokio::task::spawn_blocking(|| archive_of_our_own::get_details(document))
                        .await
                        .expect("Thread pool closed")?;

                Ok(details)
            }
            Inner::FanFictionNet { id } => {
                let url = format!("https://www.fanfiction.net/s/{}/{}", id, 1).parse::<Uri>()?;

                let body = req(&url).await?;

                let details = tokio::task::spawn_blocking(|| fanfiction::get_details(body))
                    .await
                    .expect("Thread pool closed")?;

                Ok(details)
            }
        }
    }

    pub async fn get_chapter(&mut self, chapter: u32) -> anyhow::Result<Chapter> {
        match &mut self.inner {
            Inner::ArchiveOfOurOwn { id, document } => {
                if document.is_none() {
                    let url = format!(
                        "https://archiveofourown.org/works/{}?view_full_work=true",
                        id
                    )
                    .parse::<Uri>()?;

                    let body = req(&url).await?;

                    *document = Some(Arc::new(body));
                }

                todo!()
            }
            Inner::FanFictionNet { id } => {
                let url =
                    format!("https://www.fanfiction.net/s/{}/{}", id, chapter).parse::<Uri>()?;

                let body = req(&url).await?;

                let chapter = tokio::task::spawn_blocking(|| fanfiction::get_chapter(body))
                    .await
                    .expect("Thread pool closed")?;

                Ok(chapter)
            }
        }
    }
}

enum Inner {
    ArchiveOfOurOwn {
        id: String,
        document: Option<Arc<String>>,
    },
    FanFictionNet {
        id: String,
    },
}

pub trait Site: Copy {
    fn init(self, id: impl Into<String>) -> SiteRef;
    fn init_from_url<T>(self, url: T) -> anyhow::Result<SiteRef>
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: std::error::Error;
}

impl Site for Sites {
    fn init(self, id: impl Into<String>) -> SiteRef {
        match self {
            Sites::ArchiveOfOurOwn => SiteRef {
                inner: Inner::ArchiveOfOurOwn {
                    id: id.into(),
                    document: None,
                },
            },
            Sites::FanFictionNet => SiteRef {
                inner: Inner::FanFictionNet { id: id.into() },
            },
        }
    }

    fn init_from_url<T>(self, url: T) -> anyhow::Result<SiteRef>
    where
        T: TryInto<Uri>,
        <T as TryInto<Uri>>::Error: std::error::Error,
    {
        let url: Uri = url
            .try_into()
            .map_err(|err| anyhow::anyhow!("Unable to convert string to URL: {}", err))?;

        match self {
            Sites::ArchiveOfOurOwn => archive_of_our_own::id_from_url(&url).map(|id| SiteRef {
                inner: Inner::ArchiveOfOurOwn { id, document: None },
            }),
            Sites::FanFictionNet => fanfiction::id_from_url(&url).map(|id| SiteRef {
                inner: Inner::FanFictionNet { id },
            }),
        }
    }
}
