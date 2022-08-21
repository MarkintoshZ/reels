use reels_url_pattern::UrlPattern;
use syn::parse::{Parse, ParseStream};

pub struct Args {
    pub methods: Vec<String>,
    pub url: UrlPattern,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: syn::LitStr = input.parse()?;
        let url = UrlPattern::parse(&path.value())
            .map_err(|e| syn::Error::new(path.span(), format!("Not a valid url pattern {}", e)))?;
        Ok(Self {
            methods: Vec::new(),
            url,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
