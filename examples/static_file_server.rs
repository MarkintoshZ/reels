use reels::{
    get,
    http::{HttpResponse, StatusCode},
    router::{Router, SegmentPatternValue},
    server::Server,
};
use std::path::Path;
use std::{error::Error, fs};

/// Match on any path and store the url segments into the segments argument
#[get("/<path..>")]
fn handler(path: Vec<&str>) -> HttpResponse {
    let path = path.join("/");
    let path = Path::new(&path);
    if path.is_dir() {
        render_dir(path)
    } else if path.is_file() {
        render_file(path)
    } else {
        render_unknown_path(path)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new().mount(handler)?;
    let server = Server::new(router).bind("127.0.0.1:8080")?;
    println!("Listening on http://127.0.0.1:8080");
    server.start();
    Ok(())
}

fn render_dir(path: &Path) -> HttpResponse {
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
}

fn render_file(path: &Path) -> HttpResponse {
    HttpResponse::builder()
        .status(StatusCode::OK)
        .header(
            "content-type".to_owned(),
            "application/force-download".to_owned(),
        )
        .body_bytes(fs::read(path).expect("unable to read file"))
        .finalize()
}

fn render_unknown_path(path: &Path) -> HttpResponse {
    HttpResponse::builder()
        .status(StatusCode::OK)
        .header(
            "content-type".to_owned(),
            "text/html; charset=utf-8".to_owned(),
        )
        .body(format!("Invalid path {}", path.to_str().unwrap()))
        .finalize()
}
