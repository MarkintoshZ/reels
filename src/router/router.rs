use crate::http::{response::HttpResponseBuilder, HttpRequest, HttpResponse, Method};
use crate::router::UrlPattern;
use serde::{Deserialize, Serialize};
use std::mem;

use super::url_pattern::InvalidUrlPattern;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Router {
    routes: Vec<Route>,
    // middlewares: Vec<Middleware>,
    fallback_handler: Option<HandlerPtr>,
}

impl Router {
    pub fn new() -> Router {
        Router::default()
    }

    /// Mount a service or another router on the relative path
    pub fn mount(
        mut self,
        method: Method,
        url_pattern: &str,
        handler: Handler,
    ) -> Result<Self, InvalidUrlPattern> {
        let route = Route::new(method, url_pattern, handler)?;
        self.routes.push(route);
        Ok(self)
    }

    /// Register fallback handlers
    pub fn fallback(mut self, handler: HandlerPtr) -> Self {
        self.fallback_handler = Some(handler);
        self
    }

    /// Route the request to the right handler based on the request uri prefix and method
    pub fn route(&self, req: HttpRequest) -> HttpResponse {
        let matched_route = self
            .routes
            .iter()
            .find(|route| route.match_uri(&req))
            .unwrap();

        // TODO: use fallback handler

        let response = HttpResponse::builder();
        matched_route.invoke(req, response).finalize()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Route {
    method: Method,
    url_pattern: UrlPattern,
    handler: HandlerPtr,
}

impl Route {
    pub fn new(
        method: Method,
        url_pattern: &str,
        handler: Handler,
    ) -> Result<Self, InvalidUrlPattern> {
        Ok(Self {
            method,
            url_pattern: url_pattern.try_into()?,
            handler: handler as *const () as usize,
        })
    }

    pub fn match_uri(&self, request: &HttpRequest) -> bool {
        // TODO
        self.url_pattern.match_url(&request.url).is_some() && request.method == self.method
    }

    pub fn invoke(
        &self,
        request: HttpRequest,
        response: HttpResponseBuilder,
    ) -> HttpResponseBuilder {
        let handler = unsafe {
            let pointer = self.handler as *const ();
            mem::transmute::<*const (), Handler>(pointer)
        };
        handler(request, response)
    }
}

/// Handler function
pub type Handler = fn(HttpRequest, HttpResponseBuilder) -> HttpResponseBuilder;

/// A pointer to the handler function
type HandlerPtr = usize;
