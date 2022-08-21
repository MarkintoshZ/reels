use reels::{
    http::{HttpResponse, Method, StatusCode},
    route,
    router::{Router, SegmentPatternValue},
    server::Server,
};
use std::path::Path;
use std::{error::Error, fs};

/// Match on any path and store the url segments into the segments argument
#[route("/<path..>")]
fn fallback(path: Vec<&str>) -> HttpResponse {
    let path = path.join("/");
    let path = Path::new(&path);
    if path.is_dir() {
        let mut buf = String::new();
        buf.push_str(&format!("<h1>Files at: /{}</h1>", path.to_str().unwrap()));
        buf.push_str("<ul>");
        for entry in fs::read_dir(path).expect("Unable to read dir") {
            let entry = entry.expect("Unable to read entry");
            let path = entry.path();
            let filename = entry.file_name();
            buf.push_str(&format!(
                r#"<li><a href="/{}">{}</a></li>"#,
                path.to_str().unwrap(),
                filename.to_str().unwrap()
            ));
        }
        buf.push_str("</ul>");

        HttpResponse::builder()
            .status(StatusCode::OK)
            .header(
                "content-type".to_owned(),
                "text/html; charset=utf-8".to_owned(),
            )
            .body(buf)
            .finalize()
    } else if path.is_file() {
        HttpResponse::builder()
            .status(StatusCode::OK)
            .header(
                "content-type".to_owned(),
                "application/force-download".to_owned(),
            )
            .body_bytes(fs::read(path).expect("unable to read file"))
            .finalize()
    } else {
        HttpResponse::builder()
            .status(StatusCode::OK)
            .header(
                "content-type".to_owned(),
                "text/html; charset=utf-8".to_owned(),
            )
            .body(format!("Invalid path {}", path.to_str().unwrap()))
            .finalize()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new().mount(Method::GET, "/<path..>", fallback)?;
    let server = Server::new(router).bind("127.0.0.1:8080".parse().unwrap());
    println!("Listening on http://127.0.0.1:8080");
    server.start();
    Ok(())
}
