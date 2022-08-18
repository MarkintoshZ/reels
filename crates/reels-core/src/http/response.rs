use std::{collections::HashMap, io::Write};

use super::{status::StatusCode, version::Version};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponse {
    /// The response's status
    pub status: StatusCode,

    /// The response's version
    pub version: Version,

    /// The response's headers
    pub headers: HashMap<String, String>,

    /// The response's body
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::new()
    }

    pub fn write<T: Write>(self, stream: &mut T) -> std::io::Result<()> {
        write!(stream, "HTTP/1.1 {}\r\n", self.status)?;
        self.headers.iter().for_each(|(key, value)| {
            write!(stream, "{}: {}\r\n", key, value).unwrap();
        });
        write!(stream, "\r\n")?;
        stream.write_all(&self.body)?;
        write!(stream, "\r\n")?;
        stream.flush()?;
        Ok(())
    }
}

/// Convient builder for HttpResponse objects
#[derive(Default, Debug)]
pub struct HttpResponseBuilder {
    /// The response's status
    pub status: StatusCode,

    /// The response's version
    pub version: Version,

    /// The response's headers
    pub headers: HashMap<String, String>,

    /// The response's body
    pub body: Vec<u8>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, code: StatusCode) -> Self {
        self.status = code;
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn body(mut self, content: String) -> Self {
        self.body = content.into_bytes();
        self
    }

    pub fn finalize(mut self) -> HttpResponse {
        let n = self.body.len();
        self.headers
            .insert("content-length".to_owned(), n.to_string());
        HttpResponse {
            status: self.status,
            version: self.version,
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn get_status(&self) -> &StatusCode {
        &self.status
    }

    pub fn get_status_mut(&mut self) -> &mut StatusCode {
        &mut self.status
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn get_body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}
