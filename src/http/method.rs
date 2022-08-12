use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Method {
    GET,
    PUT,
    HEAD,
    POST,
    PATCH,
    TRACE,
    DELETE,
    CONNECT,
    OPTIONS,
}

impl Method {
    fn parse(s: &str) -> Result<Self, ()> {
        match s {
            "GET" => Ok(Method::GET),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "TRACE" => Ok(Method::TRACE),
            "Delete" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            _ => Err(()),
        }
    }
}

impl TryInto<Method> for &str {
    type Error = ();

    fn try_into(self) -> Result<Method, ()> {
        Method::parse(self)
    }
}
