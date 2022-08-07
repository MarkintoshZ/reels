use http::{response, Method, Request, Response};

pub struct Router {
    routes: Vec<Route>,
    middlewares: Vec<Middleware>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn with(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn with_middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
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
