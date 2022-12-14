use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub enum Method {
    Get,
    Put,
    Head,
    Post,
    Patch,
    Trace,
    Delete,
    Connect,
    Options,
}

impl Method {
    fn parse(s: &str) -> Result<Self, ()> {
        match s {
            "GET" => Ok(Method::Get),
            "PUT" => Ok(Method::Put),
            "HEAD" => Ok(Method::Head),
            "POST" => Ok(Method::Post),
            "PATCH" => Ok(Method::Patch),
            "TRACE" => Ok(Method::Trace),
            "Delete" => Ok(Method::Delete),
            "CONNECT" => Ok(Method::Connect),
            "OPTIONS" => Ok(Method::Options),
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
