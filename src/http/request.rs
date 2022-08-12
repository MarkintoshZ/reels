use httparse::Request;
use lunatic::net;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use url::Url;

use super::{Method, Version};

const MAX_HEADER_LENGTH: usize = 32;

#[derive(Debug)]
pub struct HttpRequest {
    /// The request's method
    pub method: Method,
    /// The request's url
    pub url: Url,
    /// The request's version
    pub version: Version,
    /// The request's headers
    pub headers: HashMap<String, String>,
    /// HTTP body
    pub body: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum RequestParseError {
    SocketClosed,
    InvalidUrl,
    InvalidMethod,
    InvalidHttpRequest,
}

impl HttpRequest {
    pub fn parse(
        buf_reader: &mut BufReader<net::TcpStream>,
    ) -> Result<Option<Self>, RequestParseError> {
        let mut headers = [httparse::EMPTY_HEADER; MAX_HEADER_LENGTH];
        let mut req = Request::new(&mut headers);

        let mut header_buffer = Vec::new();

        loop {
            match buf_reader.read_until(b'\n', &mut header_buffer) {
                Ok(n) if n == 0 => return Ok(None),
                Ok(_) => {
                    let n = header_buffer.len();
                    if n >= 3 && header_buffer[n - 3] == b'\n' {
                        break;
                    }
                }
                Err(_) => return Err(RequestParseError::InvalidHttpRequest),
            }
        }

        req.parse(&header_buffer).unwrap();

        let method: Method = req.method.unwrap().try_into().unwrap();
        let version: Version = req.version.unwrap().try_into().unwrap();
        let headers: HashMap<String, String> = req
            .headers
            .into_iter()
            .map(|h| {
                (
                    h.name.to_ascii_lowercase().to_owned(),
                    std::str::from_utf8(h.value).unwrap().to_owned(),
                )
            })
            .collect();

        let host = headers.get("host").unwrap();
        let path = req.path.unwrap();
        // Parse url string into type URL and only take the path portion
        let url: Url = Url::parse(&format!("http://{}{}", host, path)).unwrap();

        let content_length = headers
            .get("content-length")
            .and_then(|s| s.parse::<usize>().ok());

        let body = content_length.map(|len| {
            let mut body_buf = Vec::new();
            body_buf.reserve_exact(len);
            buf_reader.read_exact(&mut body_buf).unwrap();
            body_buf
        });

        Ok(Some(Self {
            method,
            url,
            body,
            version,
            headers,
        }))
    }
}
