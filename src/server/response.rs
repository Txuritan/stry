use std::io::Cursor;

pub struct Response {
    inner: crate::http::Response<Cursor<Vec<u8>>>,
}

impl Response {
    pub fn Ok() -> ResponseBuilder {
        ResponseBuilder::Ok()
    }

    pub fn InternalError() -> ResponseBuilder {
        ResponseBuilder::InternalError()
    }

    pub fn Location(url: impl AsRef<[u8]>) -> Self {
        ResponseBuilder::Location(url)
    }

    pub fn into_inner(self) -> crate::http::Response<Cursor<Vec<u8>>> {
        self.inner
    }
}

pub struct ResponseBuilder {
    headers: Vec<crate::http::Header>,
    status: crate::http::StatusCode,
}

impl ResponseBuilder {
    pub fn Ok() -> Self {
        Self {
            headers: Vec::with_capacity(10),
            status: crate::http::StatusCode(200),
        }
    }

    pub fn InternalError() -> Self {
        Self {
            headers: Vec::with_capacity(10),
            status: crate::http::StatusCode(500),
        }
    }

    pub fn Location(url: impl AsRef<[u8]>) -> Response {
        Self {
            headers: Vec::with_capacity(10),
            status: crate::http::StatusCode(301),
        }
        .header("Location", url)
        .body("")
    }

    pub fn header(mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Self {
        self.headers
            .push(crate::http::Header::from_bytes(&key.as_ref()[..], &value.as_ref()[..]).unwrap());

        self
    }

    pub fn body(self, data: impl Into<String>) -> Response {
        let data = data.into();
        let data_len = data.len();

        Response {
            inner: crate::http::Response::new(
                self.status,
                self.headers,
                Cursor::new(data.into_bytes()),
                Some(data_len),
                None,
            ),
        }
    }

    pub fn html(self, data: impl Into<String>) -> Response {
        self.header("Content-Type", "text/html; charset=UTF-8")
            .body(data)
    }
}
