# Reels

An simple http library for [Lunatic](https://github.com/lunatic-solutions/lunatic)

## Goal

This is a project for me to learn more about building http library in rust. If you are looking for more sophisticated solutions, check out the official web framework by lunatic [submillisecond](https://github.com/lunatic-solutions/submillisecond) and the experimental http library [Puck](https://github.com/puck-rs/puck) that is inspired by Phoenix Liveview.

## Example

### Defining http routes

```rust
/// Match on "/users/<uid>" where uid can be parsed into a u32
#[get("/users/<uid>")]
fn user_uid(uid: u32) -> HttpResponse {
    HttpResponse::builder()
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(format!("<h1>Hi there, uid of {}!</h1>", uid).to_owned())
        .finalize()
}

/// Match on "/users/<name>" where name could be anything
#[get("/users/<name>")]
fn user(name: &str) -> HttpResponse {
    HttpResponse::builder()
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(format!("<h1>Hi there, {}!</h1>", name).to_owned())
        .finalize()
}

fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .mount(user_uid)?
        .mount(user)?
    Server::new(router)
        .bind("127.0.0.1:8080")?;
        .start();
    Ok(())
}
```

Check out the [examples](/examples) folder for more.

## Roadmap

- [x] Router support
- [x] Route macros
- [x] Route type guards
- [ ] Responder
- [ ] Middleware
- [ ] Keep alive
- [ ] Websocket
- [ ] TLS/SSL support

## License

MIT
