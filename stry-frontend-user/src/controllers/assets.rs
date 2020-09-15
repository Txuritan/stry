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
    (@internal_bytes => $mime:expr, $t:tt, $file:tt) => {
        warp::path(concat!($file, ".", $t)).and(warp::path::end()).map(|| Mime {
            body: {
                static CONTENTS: &'static [u8] =  include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file, ".", $t));

                CONTENTS
            },
            mime: $mime,
        })
    };
    (@internal_str => $mime:expr, $t:tt, $file:tt) => {
        warp::path(concat!($file, ".", $t)).and(warp::path::end()).map(|| Mime {
            body: {
                static CONTENTS: &'static str =  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file, ".", $t));

                CONTENTS
            },
            mime: $mime,
        })
    };
    (ico => $file:tt) => {{
        embed!(@internal_bytes => "image/x-icon", "ico", $file)
    }};
    (png => $file:tt) => {{
        embed!(@internal_bytes => "image/png", "png", $file)
    }};
    (svg => $file:tt) => {{
        embed!(@internal_str => "image/svg+xml", "svg", $file)
    }};
    (webmanifest => $file:tt) => {{
        embed!(@internal_str => "application/manifest+json", "webmanifest", $file)
    }};
    (xml => $file:tt) => {{
        embed!(@internal_str => "application/xml", "xml", $file)
    }};
}

pub fn assets() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(
            embed!(png => "android-chrome-192x192")
                .boxed()
                .or(embed!(png => "android-chrome-256x256"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-114x114-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-114x114"))
                .boxed()
                .boxed()
                .or(embed!(png => "apple-touch-icon-120x120-precomposed"))
                .or(embed!(png => "apple-touch-icon-120x120"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-144x144-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-144x144"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-152x152-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-152x152"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-180x180-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-180x180"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-57x57-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-57x57"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-60x60-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-60x60"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-72x72-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-72x72"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-76x76-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-76x76"))
                .boxed()
                .or(embed!(png => "apple-touch-icon-precomposed"))
                .boxed()
                .or(embed!(png => "apple-touch-icon"))
                .boxed()
                .or(embed!(xml => "browserconfig"))
                .boxed()
                .or(embed!(png => "favicon-16x16"))
                .boxed()
                .or(embed!(png => "favicon-32x32"))
                .boxed()
                .or(embed!(ico => "favicon"))
                .boxed()
                .or(embed!(png => "mstile-144x144"))
                .boxed()
                .or(embed!(png => "mstile-150x150"))
                .boxed()
                .or(embed!(svg => "safari-pinned-tab"))
                .boxed()
                .or(embed!(webmanifest => "site"))
                .boxed()
                .or(css())
                .boxed()
                .or(fonts())
                .boxed()
                .or(js())
                .boxed(),
        )
        .boxed()
}

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

pub fn fonts() -> BoxedFilter<(impl Reply,)> {
    warp::path("fonts")
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(|file: String| async move {
            match file.as_str() {
                "fontawesome-webfont.eot" => Ok(Font::new(
                    FontType::EOT,
                    resource!("assets/fonts/fontawesome-webfont.eot"),
                )),
                "fontawesome-webfont.svg" => Ok(Font::new(
                    FontType::SVG,
                    resource!("assets/fonts/fontawesome-webfont.svg"),
                )),
                "fontawesome-webfont.ttf" => Ok(Font::new(
                    FontType::TTF,
                    resource!("assets/fonts/fontawesome-webfont.ttf"),
                )),
                "fontawesome-webfont.woff" => Ok(Font::new(
                    FontType::WOFF,
                    resource!("assets/fonts/fontawesome-webfont.woff"),
                )),
                "fontawesome-webfont.woff2" => Ok(Font::new(
                    FontType::WOFF2,
                    resource!("assets/fonts/fontawesome-webfont.woff2"),
                )),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
}

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
    const fn mime(self) -> &'static str {
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
    inner: Resource<[u8]>,
}

impl Font {
    const fn new(mime: FontType, inner: Resource<[u8]>) -> Self {
        Font { mime, inner }
    }
}

impl Reply for Font {
    #[inline]
    fn into_response(self) -> Response {
        let borrow: Cow<'static, [u8]> = self.inner.into();
        let mut res = Response::new(borrow.into());

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
