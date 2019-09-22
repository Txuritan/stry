#![allow(dead_code, non_snake_case)]

use std::io::Cursor;

pub struct Response {
    inner: tiny_http::Response<Cursor<Vec<u8>>>,
}

impl Response {
    pub fn Ok() -> ResponseBuilder {
        ResponseBuilder::Ok()
    }

    pub fn NotFound() -> ResponseBuilder {
        ResponseBuilder::NotFound()
    }

    pub fn InternalError() -> ResponseBuilder {
        ResponseBuilder::InternalError()
    }

    pub fn BadRequest() -> ResponseBuilder {
        ResponseBuilder::BadRequest()
    }

    pub fn Location(url: impl AsRef<[u8]>) -> Self {
        ResponseBuilder::Location(url)
    }

    pub fn into_inner(self) -> tiny_http::Response<Cursor<Vec<u8>>> {
        self.inner
    }
}

pub struct ResponseBuilder {
    headers: Vec<tiny_http::Header>,
    status: tiny_http::StatusCode,
}

impl ResponseBuilder {
    pub fn Ok() -> Self {
        Self {
            headers: Vec::with_capacity(10),
            status: tiny_http::StatusCode(200),
        }
    }

    pub fn NotFound() -> Self {
        Self {
            headers: Vec::with_capacity(10),
            status: tiny_http::StatusCode(404),
        }
    }

    pub fn InternalError() -> Self {
        Self {
            headers: Vec::with_capacity(10),
            status: tiny_http::StatusCode(500),
        }
    }

    pub fn BadRequest() -> Self {
        Self {
            headers: Vec::with_capacity(10),
            status: tiny_http::StatusCode(400),
        }
    }

    pub fn Location(url: impl AsRef<[u8]>) -> Response {
        Self {
            headers: Vec::with_capacity(10),
            status: tiny_http::StatusCode(301),
        }
        .header("Location", url)
        .body("")
    }

    pub fn header(mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Self {
        self.headers
            .push(tiny_http::Header::from_bytes(&key.as_ref()[..], &value.as_ref()[..]).unwrap());

        self
    }

    pub fn body(self, data: impl Into<String>) -> Response {
        let data = data.into();
        let data_len = data.len();

        Response {
            inner: tiny_http::Response::new(
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

    pub fn css(self, data: impl Into<String>) -> Response {
        self.header("Content-Type", "text/css; charset=UTF-8")
            .body(data)
    }

    pub fn js(self, data: impl Into<String>) -> Response {
        self.header("Content-Type", "application/javascript; charset=UTF-8")
            .body(data)
    }

    pub fn json(self, data: impl serde::Serialize) -> Response {
        match serde_json::to_string(&data) {
            Ok(data) => self
                .header("Content-Type", "application/json; charset=UTF-8")
                .body(data),
            Err(err) => {
                log::error!("JSON error: {}", err);

                self.header("Content-Type", "application/json; charset=UTF-8")
                    .body(r#"{"status":"error","code":500,"messages":["Internal Server Error"],"data":{}}"#)
            }
        }
    }

    pub fn stream(self, data: Vec<u8>) -> Response {
        let data_len = data.len();

        Response {
            inner: tiny_http::Response::new(
                self.status,
                self.headers,
                Cursor::new(data),
                Some(data_len),
                None,
            ),
        }
    }

    pub fn html_stream(self, data: Vec<u8>) -> Response {
        self.header("Content-Type", "text/html; charset=UTF-8")
            .stream(data)
    }
}
