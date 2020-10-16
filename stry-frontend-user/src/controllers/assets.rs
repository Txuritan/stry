use {
    resource::{resource, Resource},
    std::borrow::Cow,
    warp::{
        filters::BoxedFilter,
        http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE},
        hyper::Body,
        reply::Response,
        Filter, Reply,
    },
};

macro_rules! embed {
    (@internal => $mime:expr, $t:tt, $file:tt) => {
        Ok(Mime {
            body: {
                static CONTENTS: &'static [u8] =  include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file, ".", $t));

                CONTENTS
            },
            mime: $mime,
        })
    };
    (ico => $file:tt) => {{
        embed!(@internal => "image/x-icon", "ico", $file)
    }};
    (png => $file:tt) => {{
        embed!(@internal => "image/png", "png", $file)
    }};
    (svg => $file:tt) => {{
        embed!(@internal => "image/svg+xml", "svg", $file)
    }};
    (webmanifest => $file:tt) => {{
        embed!(@internal => "application/manifest+json", "webmanifest", $file)
    }};
    (xml => $file:tt) => {{
        embed!(@internal => "application/xml", "xml", $file)
    }};
    (match $param:expr => { $( [ $token:tt ] => $file:tt , )+ $rest:ident => $body:block, }) => {{
        match $param {
            $(
                concat!($file, ".", stringify!($token)) => embed!($token => $file),
            )+
            _ => $body,
        }
    }};
}

#[inline]
pub fn assets() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(css())
        .boxed()
        .or(fonts())
        .boxed()
        .or(js())
        .boxed()
        .or(
            warp::filters::path::param::<String>().and_then(|param: String| async move {
                embed!(match param.as_str() => {
                    [ png ] => "android-chrome-192x192",
                    [ png ] => "android-chrome-256x256",
                    [ png ] => "apple-touch-icon-114x114-precomposed",
                    [ png ] => "apple-touch-icon-114x114",
                    [ png ] => "apple-touch-icon-120x120-precomposed",
                    [ png ] => "apple-touch-icon-120x120",
                    [ png ] => "apple-touch-icon-144x144-precomposed",
                    [ png ] => "apple-touch-icon-144x144",
                    [ png ] => "apple-touch-icon-152x152-precomposed",
                    [ png ] => "apple-touch-icon-152x152",
                    [ png ] => "apple-touch-icon-180x180-precomposed",
                    [ png ] => "apple-touch-icon-180x180",
                    [ png ] => "apple-touch-icon-57x57-precomposed",
                    [ png ] => "apple-touch-icon-57x57",
                    [ png ] => "apple-touch-icon-60x60-precomposed",
                    [ png ] => "apple-touch-icon-60x60",
                    [ png ] => "apple-touch-icon-72x72-precomposed",
                    [ png ] => "apple-touch-icon-72x72",
                    [ png ] => "apple-touch-icon-76x76-precomposed",
                    [ png ] => "apple-touch-icon-76x76",
                    [ png ] => "apple-touch-icon-precomposed",
                    [ png ] => "apple-touch-icon",
                    [ xml ] => "browserconfig",
                    [ png ] => "favicon-16x16",
                    [ png ] => "favicon-32x32",
                    [ ico ] => "favicon",
                    [ png ] => "mstile-144x144",
                    [ png ] => "mstile-150x150",
                    [ svg ] => "safari-pinned-tab",
                    [ webmanifest ] => "site",
                    _rest => {
                        Err(warp::reject::not_found())
                    },
                })
            }),
        )
        .boxed()
}

#[inline]
pub fn css() -> BoxedFilter<(impl Reply,)> {
    warp::path("css")
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(|file: String| async move {
            match file.as_str() {
                "easymde.css" => Ok(Css::new(resource!("assets/css/easymde.css"))),
                "font-awesome.css" => Ok(Css::new(resource!("assets/css/font-awesome.css"))),
                "stry.css" => Ok(Css::new(resource!("assets/css/stry.css"))),
                "stry.easymde.css" => Ok(Css::new(resource!("assets/css/stry.easymde.css"))),
                "tagify.css" => Ok(Css::new(resource!("assets/css/tagify.css"))),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
}

#[inline]
pub fn fonts() -> BoxedFilter<(impl Reply,)> {
    warp::path("fonts")
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(|file: String| async move {
            match file.as_str() {
                "fontawesome-webfont.eot" => Ok(Font::new(
                    FontType::EOT,
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/fonts/fontawesome-webfont.eot"
                    )),
                )),
                "fontawesome-webfont.svg" => Ok(Font::new(
                    FontType::SVG,
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/fonts/fontawesome-webfont.svg"
                    )),
                )),
                "fontawesome-webfont.ttf" => Ok(Font::new(
                    FontType::TTF,
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/fonts/fontawesome-webfont.ttf"
                    )),
                )),
                "fontawesome-webfont.woff" => Ok(Font::new(
                    FontType::WOFF,
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/fonts/fontawesome-webfont.woff"
                    )),
                )),
                "fontawesome-webfont.woff2" => Ok(Font::new(
                    FontType::WOFF2,
                    include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/fonts/fontawesome-webfont.woff2"
                    )),
                )),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
}

#[inline]
pub fn js() -> BoxedFilter<(impl Reply,)> {
    warp::path("js")
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(|file: String| async move {
            match file.as_str() {
                "easymde.js" => Ok(Js::new(resource!("assets/js/easymde.js"))),
                "mousetrap.js" => Ok(Js::new(resource!("assets/js/mousetrap.js"))),
                "stry.js" => Ok(Js::new(resource!("assets/js/stry.js"))),
                "stry-dashboard.js" => Ok(Js::new(resource!("assets/js/stry-dashboard.js"))),
                "tagify.js" => Ok(Js::new(resource!("assets/js/tagify.js"))),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
}

struct Mime<T> {
    body: T,
    mime: &'static str,
}

impl<T> Reply for Mime<T>
where
    Body: From<T>,
    T: Send,
{
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::from(self.body));

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static(self.mime));

        res
    }
}

struct Css {
    inner: Resource<[u8]>,
}

impl Css {
    const fn new(inner: Resource<[u8]>) -> Self {
        Self { inner }
    }
}

impl Reply for Css {
    #[inline]
    fn into_response(self) -> Response {
        let borrow: Cow<'static, [u8]> = self.inner.into();
        let mut res = Response::new(borrow.into());

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));

        res
    }
}

enum FontType {
    EOT,
    SVG,
    TTF,
    WOFF,
    WOFF2,
}

impl FontType {
    #[inline]
    fn mime(self) -> &'static str {
        match self {
            FontType::EOT => "application/vnd.ms-fontobject",
            FontType::SVG => "image/svg+xml",
            FontType::TTF => "font/ttf",
            FontType::WOFF => "font/woff",
            FontType::WOFF2 => "font/woff2",
        }
    }
}

struct Font {
    mime: FontType,
    inner: &'static [u8],
}

impl Font {
    const fn new(mime: FontType, inner: &'static [u8]) -> Self {
        Font { mime, inner }
    }
}

impl Reply for Font {
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::new(self.inner.into());

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static(self.mime.mime()));

        res
    }
}

struct Js {
    inner: Resource<[u8]>,
}

impl Js {
    const fn new(inner: Resource<[u8]>) -> Self {
        Self { inner }
    }
}

impl Reply for Js {
    #[inline]
    fn into_response(self) -> Response {
        let borrow: Cow<'static, [u8]> = self.inner.into();
        let mut res = Response::new(borrow.into());

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/javascript"),
        );

        res
    }
}
