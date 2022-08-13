use reels::{
    http::{HttpRequest, HttpResponseBuilder, Method, StatusCode},
    router::{PathCapture, Router, SegmentPatternValue},
    server::Server,
};
use std::error::Error;

fn index(_: PathCapture, _req: &HttpRequest, resp: HttpResponseBuilder) -> HttpResponseBuilder {
    resp.header(
        "content-type".to_owned(),
        "text/html; charset=utf-8".to_owned(),
    )
    .body("Hello world!".to_owned())
}

fn user(
    capture: PathCapture,
    _req: &HttpRequest,
    resp: HttpResponseBuilder,
) -> HttpResponseBuilder {
    if let [SegmentPatternValue::Wildcard(name)] = capture[..] {
        resp.header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(format!("<h1>Hi there, {}!</h1>", name).to_owned())
    } else {
        unreachable!();
    }
}

fn fallback(_: PathCapture, _req: &HttpRequest, resp: HttpResponseBuilder) -> HttpResponseBuilder {
    resp.status(StatusCode::NOT_FOUND)
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body("<h1>404 Not found</h1><p>Looks like you are lost</p>".to_owned())
}

fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .mount(Method::GET, "/users/<name>", user)?
        .mount(Method::GET, "/", index)?
        .mount(Method::GET, "/<rest..>", fallback)?;
    let server = Server::new(router).bind("127.0.0.1:8080".parse().unwrap());
    println!("Listening on http://127.0.0.1:8080");
    server.start();
    Ok(())
}
