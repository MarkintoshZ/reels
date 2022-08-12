use crate::http::{response::HttpResponseBuilder, HttpRequest, HttpResponse, Method};
use serde::{Deserialize, Serialize};
use std::mem;

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
    pub fn mount(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
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
    uri_prefix: String,
    handler: HandlerPtr,
}

impl Route {
    pub fn new(method: Method, uri_prefix: &str, handler: Handler) -> Self {
        Self {
            method,
            uri_prefix: uri_prefix.to_string(),
            handler: handler as *const () as usize,
        }
    }

    pub fn match_uri(&self, request: &HttpRequest) -> bool {
        request.url.path().starts_with(&self.uri_prefix) && request.method == self.method
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

/// A pointer to the handler function
type HandlerPtr = usize;
/// Handler function
type Handler = fn(HttpRequest, HttpResponseBuilder) -> HttpResponseBuilder;

pub struct Middleware {}
