use {
    crate::{query::Document, Uri},
    isahc::prelude::*,
    rand::Rng,
    std::{thread, time},
};

pub fn inner_html(
    html: &Document,
    selector: &'static str,
    name: &'static str,
) -> anyhow::Result<String> {
    html.select(selector)
        .first()
        .and_then(|sd| sd.inner_html())
        .ok_or_else(|| anyhow::anyhow!("Sector element for site {} not found: {}", name, selector))
}

pub fn string(
    html: &Document,
    selector: &'static str,
    name: &'static str,
) -> anyhow::Result<String> {
    html.select(selector)
        .first()
        .and_then(|sd| sd.text())
        .ok_or_else(|| anyhow::anyhow!("Sector element for site {} not found: {}", name, selector))
}

pub fn string_vec(
    html: &Document,
    selector: &'static str,
    name: &'static str,
) -> anyhow::Result<Vec<String>> {
    html.select(selector)
        .into_iter()
        .map(|ele| ele.text())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow::anyhow!("Sector element for site {} not found: {}", name, selector))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (X11; Linux x86_64; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);
#[cfg(all(target_os = "linux", not(target_arch = "x86_64")))]
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (X11; Linux i686; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);
#[cfg(all(target_os = "windows", not(target_arch = "x86_64")))]
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Windows NT 6.1; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
// Neither Linux nor Windows, so maybe OS X, and if not then OS X is an okay fallback.
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.10; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);

#[cfg(target_os = "android")]
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Android; Mobile; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);
#[cfg(target_os = "ios")]
const USER_AGENT: &str = concat!(
    "Mozilla/5.0 (iPhone; CPU iPhone OS 8_3 like Mac OS X; rv:63.0) Servo/1.0 Firefox/63.0 StoryDL/",
    env!("CARGO_PKG_VERSION"),
    " (txuritan@protonmail.com)"
);

pub(crate) async fn req(url: &Uri) -> anyhow::Result<String> {
    let client = HttpClient::new()?;

    let req = Request::get(url)
        .header("User-Agent", USER_AGENT)
        .body(())?;

    let mut res = client.send_async(req).await?;

    let bytes = res.text()?;

    Ok(bytes)
}

pub(crate) async fn sleep() -> anyhow::Result<()> {
    tokio::task::spawn_blocking(|| {
        let length = rand::thread_rng().gen_range(5, 10);

        tracing::info!("[util] Sleeping for {} seconds", length);

        thread::sleep(time::Duration::from_secs(length));
    })
    .await
    .expect("Thread pool closed");

    Ok(())
}

pub(crate) fn word_count(str: &str) -> u32 {
    str.split_whitespace()
        .filter(|s| match *s {
            "---" => false,
            "#" | "##" | "###" | "####" | "#####" | "######" => false,
            "*" | "**" => false,
            _ => true,
        })
        .count() as u32
}
