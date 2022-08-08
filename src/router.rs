use http::{response, Method, Request, Response};

#[derive(Default)]
pub struct Router {
    routes: Vec<Route>,
    middlewares: Vec<Middleware>,
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

    /// Attach a middleware to the router
    pub fn attach(mut self, route: Route) -> Self {
        // TODO
        self
    }

    /// Register fallback handlers
    pub fn register(mut self, route: Route) -> Self {
        // TODO
        self
    }

    pub fn route(&self, req: Request<()>) -> Response<()> {
        let mut response = Response::builder();
        (self.routes[0].handler)(req, &mut response);
        response.body(()).unwrap()
    }
}

pub struct Route {
    method: Method,
    uri_pattern: String,
    handler: Handler,
}

impl Route {
    pub fn new(method: Method, uri_pattern: String, handler: Handler) -> Self {
        Self {
            method,
            uri_pattern,
            handler,
        }
    }

    pub fn match_uri(&self, uri: &str) -> bool {
        todo!()
    }

    pub fn invoke(&self, uri: &str) {
        todo!()
    }
}

type Handler = fn(Request<()>, &mut response::Builder);

pub struct Middleware {}
