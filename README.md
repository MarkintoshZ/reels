# Reels

An experimental web framework for [Lunatic](https://github.com/lunatic-solutions/lunatic)

## Goal

The current goal of the project is to build a reliable web framework for prototyping other projects (e.g. a multiplayer game server).

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
