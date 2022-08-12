use reels::{
    http::{HttpRequest, HttpResponseBuilder, Method},
    router::{Route, Router},
    server::Server,
};

fn index(_req: HttpRequest, resp: HttpResponseBuilder) -> HttpResponseBuilder {
    resp.header(
        "content-type".to_owned(),
        "text/html; charset=utf-8".to_owned(),
    )
    .body("Hello world!".to_owned())
}

fn hi(_req: HttpRequest, resp: HttpResponseBuilder) -> HttpResponseBuilder {
    resp.header(
        "content-type".to_owned(),
        "text/html; charset=utf-8".to_owned(),
    )
    .body("Hi there!".to_owned())
}

fn main() {
    let router = Router::new()
        .mount(Route::new(Method::GET, "/hi", hi))
        .mount(Route::new(Method::GET, "/", index));
    Server::new(router)
        .bind("127.0.0.1:8080".parse().unwrap())
        .start();
    println!("Listening on port 8080");
}
