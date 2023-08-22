use crate::http::{HttpRequest, HttpResponse, Method};
pub use reels_url_pattern::{PathCapture, SegmentPattern, SegmentPatternValue, UrlPattern};
use serde::{Deserialize, Serialize};
use std::mem;

use reels_url_pattern::InvalidUrlPattern;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Router {
    routes: Vec<DefaultRoute>,
    // middlewares: Vec<Middleware>,
    fallback_handler: Option<HandlerPtr>,
}

impl Router {
    pub fn new() -> Router {
        Router::default()
    }

    /// Mount a service or another router on the relative path
    pub fn mount(mut self, handler: Handler) -> Result<Self, InvalidUrlPattern> {
        let (methods, url_pattern, handler_func) = handler();
        for method in methods.into_iter() {
            let route = DefaultRoute::new(method, url_pattern.try_into()?, handler_func);
            self.routes.push(route);
        }
        Ok(self)
    }

    /// Register fallback handlers
    pub fn fallback(mut self, handler: HandlerPtr) -> Self {
        self.fallback_handler = Some(handler);
        self
    }

    /// Route the request to the right handler based on the request uri prefix and method
    pub fn route(&self, req: HttpRequest) -> HttpResponse {
        for route in &self.routes {
            if let Some(captures) = route.match_uri(&req) {
                match route.invoke(captures, &req) {
                    Ok(response) => return response,
                    Err(SegmentTypeMissmatch) => continue,
                }
            }
        }
        // TODO: Use fallback
        HttpResponse::builder().finalize()
    }
}

pub trait Route: Sized {
    fn new(method: Method, url_pattern: UrlPattern, handler: HandlerFunc) -> Self;
    fn match_uri<'a>(&self, request: &'a HttpRequest) -> Option<PathCapture<'a>>;
    fn invoke(
        &self,
        path_capture: PathCapture,
        request: &HttpRequest,
    ) -> Result<HttpResponse, SegmentTypeMissmatch>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefaultRoute {
    method: Method,
    url_pattern: UrlPattern,
    handler: HandlerPtr,
}

impl Route for DefaultRoute {
    fn new(method: Method, url_pattern: UrlPattern, handler: HandlerFunc) -> Self {
        Self {
            method,
            url_pattern,
            handler: handler as *const () as usize,
        }
    }

    fn match_uri<'a>(&self, request: &'a HttpRequest) -> Option<PathCapture<'a>> {
        // TODO
        if request.method != self.method {
            None
        } else {
            self.url_pattern.match_url(&request.url)
        }
    }

    fn invoke<'a>(
        &self,
        path_capture: PathCapture,
        request: &HttpRequest,
    ) -> Result<HttpResponse, SegmentTypeMissmatch> {
        let handler = unsafe {
            let pointer = self.handler as *const ();
            mem::transmute::<*const (), HandlerFunc>(pointer)
        };
        handler(path_capture, request)
    }
}

/// Handler function
pub type HandlerFunc = fn(PathCapture, &HttpRequest) -> Result<HttpResponse, SegmentTypeMissmatch>;

/// Handler Trait
pub type Handler = fn() -> (Vec<Method>, &'static str, HandlerFunc);

#[derive(Debug)]
pub struct SegmentTypeMissmatch;

/// A pointer to the handler function
type HandlerPtr = usize;
