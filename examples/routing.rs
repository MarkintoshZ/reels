use reels::{
    http::{HttpResponse, Method, StatusCode},
    route,
    router::{Router, SegmentPatternValue},
    server::Server,
};
use std::error::Error;

#[route("/")]
fn index() -> HttpResponse {
    HttpResponse::builder()
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body("Hello world!".to_owned())
        .finalize()
}

/// Match on "/users/<uid>" where uid could be parsed into a u32
#[route("/users/<uid>")]
fn user_uid(uid: u32) -> HttpResponse {
    HttpResponse::builder()
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(format!("<h1>Hi there, uid of {}!</h1>", uid).to_owned())
        .finalize()
}

/// Match on "/users/<name>"
#[route("/users/<name>")]
fn user(name: &str) -> HttpResponse {
    HttpResponse::builder()
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(format!("<h1>Hi there, {}!</h1>", name).to_owned())
        .finalize()
}

/// Match on any path and store the url segments into the segments argument
#[route("/<segments..>")]
fn fallback(segments: Vec<&str>) -> HttpResponse {
    HttpResponse::builder()
        .status(StatusCode::NOT_FOUND)
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(
            format!(
                "<h1>404 Not found</h1><p>Looks like you are lost</p><p>Path: /{}</p>",
                segments.join("/")
            )
            .to_owned(),
        )
        .finalize()
}

fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .mount(Method::GET, "/users/<uid>", user_uid)?
        .mount(Method::GET, "/users/<name>", user)?
        .mount(Method::GET, "/", index)?
        .mount(Method::GET, "/<segments..>", fallback)?;
    let server = Server::new(router).bind("127.0.0.1:8080".parse().unwrap());
    println!("Listening on http://127.0.0.1:8080");
    server.start();
    Ok(())
}